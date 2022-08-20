use bat::{Input, PagingMode, PrettyPrinter};
use serde::Deserialize;
use serde_yaml::Value;
use std::{
    io::Write,
    process::{Command, Stdio},
};

fn printThemes() {
    let printer = PrettyPrinter::new();
    println!("Themes:");
    for theme in printer.themes() {
        println!("- {}", theme);
    }
}

pub fn pretty_print(string: String) {
    PrettyPrinter::new()
        .input(
            Input::from_bytes(&string.as_bytes())
                .name("diff.yaml")
                .kind("File"),
        )
        .header(true)
        .grid(true)
        .line_numbers(true)
        .use_italics(true)
        .highlight(line!() as usize)
        .language("diff")
        // .theme("sublime-snazzy")
        .paging_mode(PagingMode::Never)
        .print()
        .unwrap();
}

pub fn diff() {
    let home_dir = dirs::home_dir().unwrap();
    let target = format!(
        "{}/workspace/toca-boca/toca-days-platform/Services/travel-service/k8s/local",
        home_dir.display()
    );

    let output = Command::new("kustomize")
        .arg("build")
        .arg(target)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();

    let string = String::from_utf8(output.stdout.to_owned()).unwrap();

    for document in serde_yaml::Deserializer::from_str(string.as_str()) {
        let v = Value::deserialize(document).unwrap();
        let string = serde_yaml::to_string(&v).unwrap();

        let mut diff = Command::new("kubectl")
            .env(
                "KUBECTL_EXTERNAL_DIFF",
                format!("{}/workspace/mine/kubediff/src/diff.sh", home_dir.display()),
            )
            .arg("diff")
            .arg("-f")
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        diff.stdin
            .as_mut()
            .unwrap()
            .write(string.as_bytes())
            .unwrap();

        let diff = diff.wait_with_output().unwrap();
        let string = String::from_utf8(diff.stdout.to_owned()).unwrap();
        pretty_print(string);
    }
}
