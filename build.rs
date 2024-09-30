fn main() {
    /* 
    std::process::Command::new("cmd")
    .args(&["/C", "C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\VC\\Auxiliary\\Build\\vcvarsall.bat", "x64"])
    .status()
    .expect("Failed to set up environment variables");

    println!("cargo:rerun-if-changed=src/external/driver_control/src/lib.cpp");
    cc::Build::new()
        .cpp(true)
        //set msvc cpp version
        .flag("/std:c++latest")
        .file("src/external/driver_control/src/lib.cpp")
        .compile("drvctl");

    println!("cargo:rustc-link-lib=user32");*/
}