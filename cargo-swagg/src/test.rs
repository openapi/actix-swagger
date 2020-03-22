pub fn shot<T: crate::printer::Printable>(input: T) -> String {
    use std::fs::File;
    use std::io::{Read, Write};
    use std::process::Command;

    let dir = tempfile::tempdir().expect("Failed to create tempdir");
    let file_path = dir.path().join("source.rs");
    println!("{:?}", file_path);
    let mut file = File::create(&file_path).expect("Failed to create tempfile");

    let printed_source = input.print().to_string();

    file.write_all(printed_source.as_bytes())
        .expect("Failed to write source");

    drop(file);

    let mut child = Command::new("rustfmt")
        .arg("--emit")
        .arg("files")
        .arg(file_path.as_os_str())
        .spawn()
        .expect("Foo");

    let ecode = child.wait().expect("Failed to wait for rustfmt");

    let mut result = String::new();

    let mut file = File::open(&file_path).expect("Failed to open tempfile");
    file.sync_all().expect("Faile to sync all");
    file.read_to_string(&mut result).expect("Failed to read from tempfile");

    drop(file);
    dir.close().expect("Failed to close directory");

    result
}
