extern crate argparse;
extern crate handlebars;
extern crate resume;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;

use argparse::{ArgumentParser, Store, StoreTrue};
use handlebars::Handlebars;
use serde_yaml::{Error, Result};

use resume::data_structure::*;

fn main() {
    let person = read_person().unwrap();
    let template_md = fs::read_to_string("template.md".to_string()).unwrap_or_default();
    let template_html = fs::read_to_string("template.html".to_string()).unwrap_or_default();
    let mut handlebars = Handlebars::new();
    let md_output = handlebars.render_template(template_md.as_str(), &person).unwrap();
    let html_output = handlebars.render_template(template_html.as_str(), &person).unwrap();
    fs::write("resume.md", md_output);
    fs::write("resume.html", html_output);
}

fn read_person() -> Result<Person> {
    let f = File::open("resume.yml").unwrap();
    let reader = BufReader::new(f);
    let p: Result<Person> =
        serde_yaml::from_reader(reader);
    p
}
