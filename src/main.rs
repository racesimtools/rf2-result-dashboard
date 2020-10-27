extern crate serde;
extern crate quick_xml;

use serde::Deserialize;
use quick_xml::de::{from_str, DeError};

#[derive(Debug, Deserialize, PartialEq)]
struct Link {
    rel: String,
    href: String,
    sizes: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Lang {
    En,
    Fr,
    De,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Head {
    title: String,
    #[serde(rename = "link", default)]
    links: Vec<Link>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Script {
    src: String,
    integrity: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Body {
    #[serde(rename = "script", default)]
    scripts: Vec<Script>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Html {
    lang: Option<String>,
    head: Head,
    body: Body,
}

fn crates_io() -> Result<Html, DeError> {
    let xml = "<!DOCTYPE html>
        <html lang=\"en\">
          <head>
            <meta charset=\"utf-8\">
            <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">
            <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">

            <title>crates.io: Rust Package Registry</title>


        <!-- EMBER_CLI_FASTBOOT_TITLE --><!-- EMBER_CLI_FASTBOOT_HEAD -->
        <link rel=\"manifest\" href=\"/manifest.webmanifest\">
        <link rel=\"apple-touch-icon\" href=\"/cargo-835dd6a18132048a52ac569f2615b59d.png\" sizes=\"227x227\">

            <link rel=\"stylesheet\" href=\"/assets/vendor-8d023d47762d5431764f589a6012123e.css\" integrity=\"sha256-EoB7fsYkdS7BZba47+C/9D7yxwPZojsE4pO7RIuUXdE= sha512-/SzGQGR0yj5AG6YPehZB3b6MjpnuNCTOGREQTStETobVRrpYPZKneJwcL/14B8ufcvobJGFDvnTKdcDDxbh6/A==\" >
            <link rel=\"stylesheet\" href=\"/assets/cargo-cedb8082b232ce89dd449d869fb54b98.css\" integrity=\"sha256-S9K9jZr6nSyYicYad3JdiTKrvsstXZrvYqmLUX9i3tc= sha512-CDGjy3xeyiqBgUMa+GelihW394pqAARXwsU+HIiOotlnp1sLBVgO6v2ZszL0arwKU8CpvL9wHyLYBIdfX92YbQ==\" >


            <link rel=\"shortcut icon\" href=\"/favicon.ico\" type=\"image/x-icon\">
            <link rel=\"icon\" href=\"/cargo-835dd6a18132048a52ac569f2615b59d.png\" type=\"image/png\">
            <link rel=\"search\" href=\"/opensearch.xml\" type=\"application/opensearchdescription+xml\" title=\"Cargo\">
          </head>
          <body>
            <!-- EMBER_CLI_FASTBOOT_BODY -->
            <noscript>
                <div id=\"main\">
                    <div class='noscript'>
                        This site requires JavaScript to be enabled.
                    </div>
                </div>
            </noscript>

            <script src=\"/assets/vendor-bfe89101b20262535de5a5ccdc276965.js\" integrity=\"sha256-U12Xuwhz1bhJXWyFW/hRr+Wa8B6FFDheTowik5VLkbw= sha512-J/cUUuUN55TrdG8P6Zk3/slI0nTgzYb8pOQlrXfaLgzr9aEumr9D1EzmFyLy1nrhaDGpRN1T8EQrU21Jl81pJQ==\" ></script>
            <script src=\"/assets/cargo-4023b68501b7b3e17b2bb31f50f5eeea.js\" integrity=\"sha256-9atimKc1KC6HMJF/B07lP3Cjtgr2tmET8Vau0Re5mVI= sha512-XJyBDQU4wtA1aPyPXaFzTE5Wh/mYJwkKHqZ/Fn4p/ezgdKzSCFu6FYn81raBCnCBNsihfhrkb88uF6H5VraHMA==\" ></script>

          </body>
        </html>
}";
    let html: Html = from_str(xml)?;
    assert_eq!(&html.head.title, "crates.io: Rust Package Registry");
    Ok(html)
}

fn main() -> () {
    let res = crates_io();
    match res {
        Ok(html) => println!("parsed: {:?}", html),
        Err(e) => println!("error: {:?}", e),
    }
    
}
