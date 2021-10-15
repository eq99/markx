use clap::{App, Arg};
use markx::html::mark2html;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use tera::{Context, Tera};

fn main() -> std::io::Result<()> {
    let opts = App::new("markx")
        .version("1.0")
        .author("Qiansen Zhou <zuiaiqiansen@163.com>")
        .about("A markdown paser.")
        .arg_from_usage("-f, --file=[FILE] 'The markdown file to be parsed, The out file will be the same name but with .html extension.'")
        .get_matches();

    let mut input_file_name = String::from("");
    if let Some(f) = opts.value_of("file") {
        input_file_name = String::from(f);
    }

    input_file_name = String::from("tests/test4.md");
    let mut outfile_name = PathBuf::from(&input_file_name);
    outfile_name.set_extension("html");
    let input = fs::read_to_string(&input_file_name).unwrap();
    let _html = mark2html(&input);
    println!("{}", _html);

    // Use globbing
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    // Using the tera Context struct
    let mut context = Context::new();
    context.insert("_html", &_html);

    let html_file = tera.render("index.html", &context).unwrap();
    println!("{:?}", html_file);

    fs::write(outfile_name, html_file).expect("faild write to html");

    Ok(())
}
