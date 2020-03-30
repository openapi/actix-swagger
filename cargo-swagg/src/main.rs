fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let opts = clap::App::new("cargo-swagg")
        .author("Sergey Sova")
        .about("Generate actix-web code from openapi3 specification from CLI")
        .arg(
            clap::Arg::with_name("source")
                .help("Path to openapi3 specification file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("out-file")
                .long("out-file")
                .required(false)
                .help("Where to write rust code")
                .takes_value(true),
        )
        .get_matches();

    let content = std::fs::read_to_string(
        &opts
            .value_of("source")
            .expect("Pass file with openapi3 specification"),
    )?;

    let source_code = swagg::to_string(&content, swagg::Format::Yaml).unwrap();

    let code = format!("{}", source_code);
    if let Some(file) = opts.value_of("out-file") {
        std::fs::write(file, code).expect("Failed to write rust code to out file");
    } else {
        println!("{}", code);
    }

    Ok(())
}
