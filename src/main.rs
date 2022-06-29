use std::collections::HashMap;
use std::error::Error;
use std::io;
use winreg::enums::*;
use winreg::RegKey;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use libloading::{Library, Symbol};
use std::ffi::{CString};

struct SQLBaseProcess {
    id: i32,
    active: bool,
}

struct SQLBaseCursor {
    pid: i32,
    db: String
}

struct SQLBaseConfig {
    server: String,
    boot_time: String,
    sqlbase_version: String,
    online: bool
}

struct Stats(f64, f64);

impl Stats {
    fn increment(&mut self, active: bool) {
        if active {
            self.1 += 1f64;
        } else {
            self.0 += 1f64;
        }
    }
}

type SQLCSV = Symbol<'static, extern fn(*mut i16, *const i8, *const i8) -> i16>;
type SQLDSV = Symbol<'static, extern fn(i16) -> i16>;
type SQLGSI = Symbol<'static, extern fn(i16, i32, *const i8, usize, *mut i16) -> i16>;
type SQLDBN = Symbol<'static, extern fn(*const i8, *const i8, usize) -> i16>;

const BUFFER_SIZE: usize = 60000;

fn sql_error() -> String {
    String::from("2 \"SQLBase Error\" - ")
}

fn install_location() -> Result<String, Box<dyn std::error::Error>> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut install_location: String = String::new();
    
    for key in hklm.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall")?
        .enum_keys()
        .map(|k| k.unwrap())
    {
        let subkey = hklm.open_subkey(format!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\{}", key))?;
        let name: String = match subkey.get_value("DisplayName") {
            Ok(v) => v,
            _ => String::new()
        };

        if name.starts_with("SQLBase") {
            install_location = subkey.get_value("InstallLocation")?;
            break;
        }
    }

    Ok(install_location)
}

fn read_server_name(loc: &str) -> Result<String, Box<dyn Error>> {
    let filename = format!("{}\\sql.ini", loc);
    let file = File::open(&filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines().map(|l| l.unwrap()) {
        if line.starts_with("servername=") {
            let server = line.split("=").collect::<Vec<&str>>()[1].split(",").collect::<Vec<&str>>()[0].to_string();
            return Ok(server);
        }
    }

    Err(Box::new(io::Error::new(io::ErrorKind::UnexpectedEof, "Servername was not found in sql.ini")))
}

fn extract_string(buf: &[u8], start: usize) -> String {
    let mut s = String::new();
    for (i, b) in buf.iter().enumerate() {
        if i < start {
            continue;
        }

        if *b == 0 {
            break;
        }

        s.push(*b as char);
    }

    s
}

fn extract_strings(buf: &[u8], mut start: usize, max_length: usize) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    let mut s = extract_string(buf, start);
    while s != "" && start < max_length {
        results.push(s.clone());
        start += s.chars().count() + 1;
        s = extract_string(buf, start);
    }

    results
}

fn buffer_to_unsigned_byte(buf: &[i8]) -> &[u8] {
    let u8slice : &[u8] = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u8, buf.len()) };
    u8slice
}

fn get_database_names(f: &SQLDBN, server: &str) -> Vec<String> {
    let server_ptr = CString::new(server).expect("");
    let buffer: [i8; BUFFER_SIZE] = [0; BUFFER_SIZE];

    let con = f(server_ptr.as_ptr(), buffer.as_ptr(), BUFFER_SIZE);
    if con != 0 {
        println!("{}Failed to load databases. Code {}", sql_error(), con); 
        return Vec::new();
    }

    let buf = buffer_to_unsigned_byte(&buffer);
    extract_strings(&buf, 0, BUFFER_SIZE)
}

fn get_processes(f: &SQLGSI, handle: i16) -> Vec<SQLBaseProcess> {
    let mut results: Vec<SQLBaseProcess> = Vec::new();
    let buffer: [i8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut buf_length: i16 = 0;

    let con = f(handle, 32 | 32768, buffer.as_ptr(), BUFFER_SIZE, &mut buf_length);
    if con != 0 {
        println!("{}Failed to load processes. Code {}", sql_error(), con); 
        return results;
    }

    let buf = buffer_to_unsigned_byte(&buffer);
    let mut i = 0;

    while i < buf_length - 160 {
        results.push(SQLBaseProcess {
            id: buffer[(i + 22) as usize] as i32,
            active: extract_string(&buf, (i + 80) as usize) == "performing request",
        });

        i += 160;
    }

    results
}

fn get_cursors(f: &SQLGSI, handle: i16) -> Vec<SQLBaseCursor> {
    let mut results: Vec<SQLBaseCursor> = Vec::new();
    let buffer: [i8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut buf_length: i16 = 0;

    let con = f(handle, 2, buffer.as_ptr(), BUFFER_SIZE, &mut buf_length);
    if con != 0 {
        println!("{}Failed to load cursors. Code {}", sql_error(), con); 
        return results;
    }

    let buf = buffer_to_unsigned_byte(&buffer);
    let mut i = 0;
    while i < buf_length - 160 {
        results.push(SQLBaseCursor {
            pid: buffer[(i + 29) as usize] as i32,
            db: extract_string(&buf, (i + 52) as usize)
        });

        i += 60;
    }
    results
}

fn get_sqlbase_configuration(f: &SQLGSI, handle: i16) -> Option<SQLBaseConfig> {
    let buffer: [i8; 512] = [0; 512];
    let mut buf_length: i16 = 0;
    
    let con = f(handle, 8 | 32768, buffer.as_ptr(), 512, &mut buf_length);
    if con != 0 {
        println!("{}Failed to load configuration. Code {}", sql_error(), con); 
        return None;
    }
    
    let buf = buffer_to_unsigned_byte(&buffer);
    Some(SQLBaseConfig {
        boot_time: extract_string(&buf, 324),
        online: buffer[391] == 1,
        server: extract_string(&buf, 26),
        sqlbase_version: extract_string(&buf, 371)
    })
}

fn main() -> io::Result<()> {
    let install_dir: String = match install_location() {
        Ok(val) => val,
        Err(e) => { 
            println!("{}Failed to load SQLBase directory. {}", sql_error(), e); 
            panic!()
        }
    };

    let server = match read_server_name(&install_dir) {
        Ok(val) => val,
        Err(e) => { 
            println!("{}Faild to load servername. {}", sql_error(), e);
            String::new() 
        }
    };

    let dll_name = format!("{}sqlwntm.dll", install_dir);

    let lib: Library = unsafe { Library::new(dll_name).unwrap() };
    let sqlcsv = unsafe { lib.get::<SQLCSV>(b"sqlcsv").unwrap() };
    let sqldsv = unsafe { lib.get::<SQLDSV>(b"sqldsv").unwrap() };
    let sqlgsi = unsafe { lib.get::<SQLGSI>(b"sqlgsi").unwrap() };
    let sqldbn = unsafe { lib.get::<SQLDBN>(b"sqldbn").unwrap() };

    let mut handle: i16 = 0;
    let server_ptr = CString::new(server.clone()).unwrap();
    let password_ptr = CString::new("").unwrap();

    let con = sqlcsv(&mut handle, server_ptr.as_ptr(), password_ptr.as_ptr());
    if con != 0 {
        println!("{}Failed to connect to server. Code {}", sql_error(), con);
        return Ok(())
    }

    let conf = get_sqlbase_configuration(&sqlgsi, handle).unwrap();

    let server_status = match conf.online {
        true => "0",
        false => "2"
    };

    println!("{} \"SQLBase\" - Server: {}, Version: {}, Start: {}", server_status, conf.server, conf.sqlbase_version, conf.boot_time);

    let names = get_database_names(&sqldbn, &server[..]);
    let mut stats = HashMap::new();

    for name in names {
        println!("0 \"SQLBase {} Database\" - Status: ONLINE", name);
        stats.insert(name, Stats(0f64, 0f64));
    }

    let mut processes = get_processes(&sqlgsi, handle);
    let cursors = get_cursors(&sqlgsi, handle);

    for proc in processes.iter_mut() {
        let db = match cursors.iter().find(|x| x.pid == proc.id) {
            Some(c) => &c.db,
            None => continue
        };

        let entry = match stats.get_mut(db) {
            Some(v) => v,
            None => continue
        };

        entry.increment(proc.active);
    }
    let con = sqldsv(handle);
    if con != 0 {
        println!("{}Failed to disconnect from server. Code {}", sql_error(), con);
        return Ok(())
    }

    for (k, s) in stats.iter() {
        let plt = (s.0 + s.1) * 0.2;
        let put = (s.0 + s.1) * 0.4;

        let status = match (s.0 + s.1) as i32 {
            0..=4 => "0",
            _ => "P"
        };
        
        println!("{} \"SQLBase {} Processes\" count={};{};{} {} active processes, {} idle processes", status, k, s.1, plt, put, s.1, s.0);
    }

    let _ = io::stdout().flush();

    Ok(())
}
