fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    env_logger::init();
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

    let path = opts
        .value_of("source")
        .expect("Pass file with openapi3 specification");

    let path = std::path::Path::new(&path);

    let content = std::fs::read_to_string(&path)?;

    let format = match path.extension().and_then(|ext| ext.to_str()) {
        Some("yaml") | Some("yml") | None => swagg::Format::Yaml,
        Some("json") => swagg::Format::Json,
        Some(ext) => panic!("Unexpected source extension {}", ext),
    };

    let source_code = swagg::to_string(&content, format).unwrap();

    let code = format!("{}", source_code);
    if let Some(file) = opts.value_of("out-file") {
        std::fs::write(file, code).expect("Failed to write rust code to out file");
    } else {
        println!("{}", code);
    }

    Ok(())
}
