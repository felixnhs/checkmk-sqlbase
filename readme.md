<div id="top"></div>

[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
<h3 align="center">check<b>mk</b> SQLBase</h3>

  <p align="center">
    check<b>mk</b> local check to monitor your SQLBase server
    <br />
    <a href="https://github.com/Zedane/checkmk-sqlbase/blob/master/readme.md"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/Zedane/checkmk-sqlbase/issues">Report Bug</a>
    ·
    <a href="https://github.com/Zedane/checkmk-sqlbase/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

Monitor your SQLBase server from your *checkmk dashboard*. This check works similar to the official MSSQL plugin, but is currently limited to a few services.

The check lets you monitor:
* general Database information
* databases on that server
* database processes 


### Built With

* [Rust](https://rust-lang.org//)
* [SQLBase](https://www.opentext.com/products-and-solutions/products/specialty-technologies/opentext-gupta-development-tools-databases/opentext-gupta-sqlbase)


<!-- GETTING STARTED -->
## Getting Started

This is a [local check](https://docs.checkmk.com/latest/en/localchecks.html) plugin. It needs to be applied onto every server, that you want to monitor.

### Prerequisites

You will need to build the binary. Make sure you have [cargo](https://crates.io/) installed and follow the istructions below.

Install the checkmk agent on your server and add it to your sites inventory, if you have not already done so. Follow the [checkmk docs](https://docs.checkmk.com/latest/en/agent_windows.html#install) to do so.

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/Zedane/checkmk-sqlbase.git
   ```
2. Open your command line or terminal in the projects directory and use [cargo](https://crates.io/) to build the binary:
   ```sh
   cargo build --release
   ```
3. Copy the executable into the director `C:\ProgramData\checkmk\agent\local`. 
4. On your checkmk site, edit the host services and select the services you want to monitor.


<!-- LICENSE -->
## License

Distributed under the MIT License. See [LICENSE][license-url] for more information.


<!-- CONTACT -->
## Contact

Project Link: [https://github.com/Zedane/checkmk-sqlbase](https://github.com/Zedane/checkmk-sqlbase)


<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [checkmk documentation](https://docs.checkmk.com/latest/en/)
* [Learn rust](https://doc.rust-lang.org/book/)
* [SQLBase API reference](https://manualzz.com/doc/38128556/sqlbase-sql-application-programming-interface-reference)


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[issues-shield]: https://img.shields.io/github/issues/Zedane/checkmk-sqlbase.svg?style=for-the-badge
[issues-url]: https://github.com/Zedane/checkmk-sqlbase/issues
[license-shield]: https://img.shields.io/github/license/Zedane/checkmk-sqlbase.svg?style=for-the-badge
[license-url]: https://github.com/Zedane/checkmk-sqlbase/blob/master/LICENSE