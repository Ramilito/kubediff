use std::process::{Child, Command, Stdio};

pub fn get_diff() -> Child {
    Command::new("kubectl")
        .env("KUBECTL_EXTERNAL_DIFF", format!("{}", get_script()))
        .arg("diff")
        .arg("-f")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
}

pub fn get_build(target: &String) -> String {
    let output = Command::new("kustomize")
        .arg("build")
        .arg(target)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();

    String::from_utf8(output.stdout.to_owned()).unwrap()
}

pub fn get_script() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.push("diff.sh");
    path.to_str().unwrap().to_string()
}

pub fn get_resources(resource: &str) -> String {
    let args = vec![
        "get",
        "--ignore-not-found",
        resource,
        "-A",
        "-o",
        "yaml", // "jsonpath-as-json='{.items[*]['.metadata.name', '.kind']}'",
    ];
    let mut output = get_kubectl(args);
    output = output.trim().to_string();
    output
}

pub fn get_api_resources() -> Vec<String> {
    let args = vec!["api-resources", "--verbs=list", "-o", "name"];
    let output = get_kubectl(args);
    let mut results: Vec<String> = output
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    results.sort();
    results
}

fn get_kubectl(args: Vec<&str>) -> String {
    let output = Command::new("kubectl")
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();

    String::from_utf8(output.stdout.to_owned()).unwrap()
}
