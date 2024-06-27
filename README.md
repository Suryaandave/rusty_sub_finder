# RustySubFinder

RustySubFinder is a fast and efficient subdomain finder written in Rust. It uses asynchronous HTTP requests to quickly check for the existence of subdomains for a given target domain.

## Features

- Asynchronous requests for improved performance
- Includes a pre-populated list of 5000 top subdomains (`subdomains-top1million-5000`)
- Easy to use
- Written in Rust for safety and speed

## Installation

1. Ensure you have Rust and Cargo installed. You can install Rust from (https://www.rust-lang.org/tools/install).

2. Clone the repository:

   ```sh
   git clone https://github.com/Suryaandave/rusty_sub_finder.git
   cd RustySubFinder
Build the project:


$ cargo run --release
Enter website name [in form of google.com]: example.com
Subdomain exists: www.example.com
Subdomain exists: mail.example.com
