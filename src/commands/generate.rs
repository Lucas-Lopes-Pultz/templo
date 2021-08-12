use crate::core::template::generator::gen_template;
use crate::core::utils::errors::invalid_input_error;
use crate::{
    cli::output::messages::error::{
        INVALID_DIRECTORY_PATH_NAME, INVALID_DIRECTORY_PATH_TYPE, INVALID_TEMPLATE_NAME,
    },
    core::repository::local,
};
use std::{fs, io::Error, path::Path, time::Instant};

pub fn generate(args: &[String]) -> Result<(), Error> {
    local::create()?;

    if args.len() < 1 {
        return Err(invalid_input_error(INVALID_TEMPLATE_NAME));
    }

    if args.len() < 2 {
        return Err(invalid_input_error(INVALID_DIRECTORY_PATH_NAME));
    }

    let template_name = &args[0];
    let directory = Path::new(&args[1]);

    if directory.extension() != None {
        return Err(invalid_input_error(INVALID_DIRECTORY_PATH_TYPE));
    }

    if !directory.exists() {
        fs::create_dir_all(directory)?;
    }

    let start = Instant::now(); // start timing process
    let template = local::get_template(&template_name)?;

    gen_template(template, directory)?;
    println!("Template \"{}\" was generated.", template_name);

    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));
    Ok(())
}
