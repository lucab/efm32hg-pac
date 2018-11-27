use form::create_directory_structure;
use quote::{multi_zip_expr, nested_tuples_pat, pounded_var_names, quote, quote_each_token};
use rustfmt_nightly::{Config, Input, Session};
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::PathBuf;
use svd2rust::{generate, target::Target, util::build_rs, util::SvdError};
use svd_parser;

pub fn main() -> Result<(), SvdError> {
    //parse xml
    let xml = read_to_string("svd/EFM32HG309F64.svd")?;
    let device = svd_parser::parse(&xml);

    //parse svd
    let mut device_x = String::new();
    let items = generate::device::render(&device, &Target::CortexM, true, &mut device_x)?;

    //save other files
    writeln!(File::create("device.x")?, "{}", device_x)?;
    writeln!(File::create("build.rs")?, "{}", build_rs())?;

    //form lib.rs and save
    let lib = format!("{}", quote!(#(#items)*));
    create_directory_structure("src/", lib)?;

    //rustfmt saved files
    let files = ["build.rs", "src/lib.rs"].iter().map(|a| PathBuf::from(a));
    let mut buf = Vec::<u8>::new();
    let mut session = Session::new(Config::default(), Some(&mut buf));

    for path in files {
        session.format(Input::File(path)).unwrap();
    }

    Ok(())
}
