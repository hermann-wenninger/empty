use std::env;
use std::path::Path;


fn main() {
    // Compile our SQL
    cornucopia();
}

fn cornucopia() {
    // For the sake of simplicity, this example uses the defaults.
    let queries_path = "queries";
    let binding = env::var_os("DATABASE_URL").expect("XXXXXXXXXXXXXXXXXX");
    let db_url = binding;

    // Debug-Ausgabe (falls benötigt)
    //println!("cargo:warning=Using DATABASE_URL: {:?}", &db_url);

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let file_path = Path::new(&out_dir).join("cornucopia.rs");

    //let db_url = env::var_os("DATABASE_URL").unwrap();
    //let db_url = Some("postgresql://postgres:23hermann75@#*@127.0.0.1:5432/nails?sslmode=disable").expect("DATABASE_URL must be set");
    // Rerun this build script if the queries or migrations change.
    println!("cargo:rerun-if-changed={queries_path}");

    // Call cornucopia. Use whatever CLI command you need.
    let output = std::process::Command::new("cornucopia")
        .arg("-q")
        .arg(queries_path)
        .arg("--serialize")
        .arg("-d")
        .arg(&file_path)
        .arg("live")
        .arg(db_url)
        .output()
        .unwrap();

    // If Cornucopia couldn't run properly, try to display the error.
    if !output.status.success() {
        panic!("{}", &std::str::from_utf8(&output.stderr).unwrap());
    }
}