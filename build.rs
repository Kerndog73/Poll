fn main() {
    println!("cargo:rerun-if-changed=client/public/config_cat.html");
    println!("cargo:rerun-if-changed=client/public/config_num.html");
    println!("cargo:rerun-if-changed=client/public/favicon.ico");
    println!("cargo:rerun-if-changed=client/public/home.html");
    println!("cargo:rerun-if-changed=client/public/respond_cat.html");
    println!("cargo:rerun-if-changed=client/public/respond_num.html");
    println!("cargo:rerun-if-changed=client/public/results.html");
    println!("cargo:rerun-if-changed=client/public/run.html");
    println!("cargo:rerun-if-changed=client/public/success.html");

    println!("cargo:rerun-if-changed=client/src/pages/common/index.js");
    println!("cargo:rerun-if-changed=client/src/pages/config_cat/index.js");
    println!("cargo:rerun-if-changed=client/src/pages/config_num/index.js");
    println!("cargo:rerun-if-changed=client/src/pages/home/index.js");
    println!("cargo:rerun-if-changed=client/src/pages/respond_cat/index.js");
    println!("cargo:rerun-if-changed=client/src/pages/respond_num/index.js");
    println!("cargo:rerun-if-changed=client/src/pages/results/index.js");
    println!("cargo:rerun-if-changed=client/src/pages/run/index.js");
    println!("cargo:rerun-if-changed=client/src/pages/success/index.js");

    println!("cargo:rerun-if-changed=client/src/scss/common.scss");
    println!("cargo:rerun-if-changed=client/src/scss/custom.scss");

    println!("cargo:rerun-if-changed=client/webpack.config.js");

    let build = match std::env::var("PROFILE").unwrap().as_str() {
        "debug" => "build-dev",
        "release" => "build-prod",
        _ => panic!()
    };

    let status = std::process::Command::new("npm")
        .arg("run")
        .arg(build)
        .current_dir("client")
        .status()
        .unwrap();
    assert!(status.success());
}
