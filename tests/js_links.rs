/// Runs tests/links.js via Node.js to verify the client-side relative-link
/// behaviour (resolveRelativePath, shouldIntercept, extractFilePart).
#[test]
fn js_relative_link_behaviour() {
    let output = std::process::Command::new("node")
        .arg("tests/links.js")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("node not found — install Node.js to run JS tests");

    let stdout = String::from_utf8_lossy(&output.stdout);
    print!("{stdout}");

    assert!(
        output.status.success(),
        "JS link tests failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
