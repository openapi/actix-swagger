pub fn pretty(input: String) -> String {
    use std::fs::File;
    use std::io::{Read, Write};
    use std::process::Command;

    let dir = tempfile::tempdir().expect("Failed to create tempdir");
    let file_path = dir.path().join("source.rs");
    println!("{:?}", file_path);
    let mut file = File::create(&file_path).expect("Failed to create tempfile");

    file.write_all(input.as_bytes())
        .expect("Failed to write source");

    let mut child = Command::new("rustfmt")
        .arg("--emit")
        .arg("files")
        .arg(file_path.as_os_str())
        .spawn()
        .expect("Rustfmt failed to run");

    let _ = child.wait().expect("Failed to wait for rustfmt");

    let mut result = String::new();

    let mut file = File::open(&file_path).expect("Failed to open tempfile");
    file.sync_all().expect("Faile to sync all");
    file.read_to_string(&mut result)
        .expect("Failed to read from tempfile");

    dir.close().expect("Failed to close directory");

    result.trim().to_owned()
}

pub fn shot<T: crate::printer::Printable>(input: T) -> String {
    pretty(input.print().to_string())
}
