// Awesome simple way to generate a tag based on current cargo package version
fn main() {
    let version = std::env!("CARGO_PKG_VERSION");
    println!(
        r#"git tag --sign --create-reflog --message "Release v{}" v{}"#,
        version, version
    );
}
