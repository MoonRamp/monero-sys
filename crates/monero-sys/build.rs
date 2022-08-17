use std::{env, process::Command};

fn main() -> anyhow::Result<()> {
    let path = env::current_dir().unwrap().join("monero/contrib/depends");
    let mut cmd = Command::new("sh");
    cmd.args(["-c", "exec \"$0\" \"$@\""]).arg("make");
    cmd.current_dir(&path);
    cmd.status()?;
    //let dep_dst = autotools::Config::new(V).fast_build(true).build();

    let dst = cmake::Config::new("monero")
        .define("BUILD_TESTS", "OFF")
        .define("STATIC", "ON")
        .define("BUILD_GUI_DEPS", "ON")
        .define(
            "CMAKE_TOOLCHAIN_FILE",
            "monero/contrib/depends/share/toolchain.cmake",
        )
        .define("CMAKE_BUILD_TYPE", "Release")
        .build_target("wallet_api")
        .build();

    let monero_src = std::path::PathBuf::from("monero/src");
    let monero_external = std::path::PathBuf::from("monero/external");
    let monero_external_easylog = std::path::PathBuf::from("monero/external/easylogging++");
    let contrib_epee = std::path::PathBuf::from("monero/contrib/epee/include");
    let mut b = autocxx_build::Builder::new(
        "src/lib.rs",
        &[
            &monero_src,
            &monero_external,
            &monero_external_easylog,
            &contrib_epee,
        ],
    )
    .build()?;
    b.flag_if_supported("-std=c++14").compile("monero-sys");

    // Link xmr libs
    let libxmr = dst.display();
    println!("cargo:rustc-link-search=native={}/build/lib", libxmr);
    println!("cargo:rustc-link-lib=static=wallet");
    println!("cargo:rustc-link-lib=static=wallet_api");

    println!("cargo:rustc-link-search={}/build/src/device", libxmr);
    println!("cargo:rustc-link-search={}/build/src/device_trezor", libxmr);
    println!("cargo:rustc-link-search={}/build/contrib/epee/src", libxmr);
    println!(
        "cargo:rustc-link-search={}/build/external/db_drivers/liblmdb",
        libxmr
    );
    println!(
        "cargo:rustc-link-search={}/build/external/easylogging++",
        libxmr
    );
    println!("cargo:rustc-link-search={}/build/external/randomx", libxmr);
    println!("cargo:rustc-link-search={}/build/src/checkpoints", libxmr);
    println!("cargo:rustc-link-search={}/build/src/common", libxmr);
    println!("cargo:rustc-link-search={}/build/src/crypto", libxmr);
    println!("cargo:rustc-link-search={}/build/src/crypto/wallet", libxmr);
    println!(
        "cargo:rustc-link-search={}/build/src/cryptonote_basic",
        libxmr
    );
    println!(
        "cargo:rustc-link-search={}/build/src/cryptonote_core",
        libxmr
    );
    println!(
        "cargo:rustc-link-search={}/build/src/cryptonote_protocol",
        libxmr
    );
    println!("cargo:rustc-link-search={}/build/src/mnemonics", libxmr);
    println!("cargo:rustc-link-search={}/build/src/multisig", libxmr);
    println!("cargo:rustc-link-search={}/build/monero_crypto_src", libxmr);
    println!("cargo:rustc-link-search={}/build/src/net", libxmr);
    println!("cargo:rustc-link-search={}/build/src/ringct", libxmr);
    println!("cargo:rustc-link-search={}/build/src/rpc", libxmr);
    println!("cargo:rustc-link-search={}/build/src", libxmr);
    println!("cargo:rustc-link-lib=static=device");
    println!("cargo:rustc-link-lib=static=device_trezor");
    println!("cargo:rustc-link-lib=static=checkpoints");
    println!("cargo:rustc-link-lib=static=cncrypto");
    println!("cargo:rustc-link-lib=static=common");
    println!("cargo:rustc-link-lib=static=cryptonote_basic");
    println!("cargo:rustc-link-lib=static=cryptonote_core");
    println!("cargo:rustc-link-lib=static=cryptonote_format_utils_basic");
    //println!("cargo:rustc-link-lib=cryptonote_protocol");
    println!("cargo:rustc-link-lib=static=easylogging");
    println!("cargo:rustc-link-lib=static=epee");
    //println!("cargo:rustc-link-lib=epee_readline");
    println!("cargo:rustc-link-lib=static=lmdb");
    println!("cargo:rustc-link-lib=static=mnemonics");
    println!("cargo:rustc-link-lib=static=multisig");
    println!("cargo:rustc-link-lib=static=net");
    println!("cargo:rustc-link-lib=static=randomx");
    println!("cargo:rustc-link-lib=static=ringct");
    println!("cargo:rustc-link-lib=static=ringct_basic");
    println!("cargo:rustc-link-lib=static=rpc_base");
    println!("cargo:rustc-link-lib=static=wallet-crypto");
    println!("cargo:rustc-link-lib=static=version");

    //// Link 3rd party libs
    //system_deps::Config::new().probe().unwrap();
    //link_boost();

    println!("cargo:rerun-if-changed=src/lib.rs");
    Ok(())
}

//fn link_boost() {
//    let boost_libs = [
//        "boost_chrono",
//        "boost_date_time",
//        "boost_filesystem",
//        #[cfg(target_os = "linux")]
//        "boost_locale",
//        #[cfg(target_os = "macos")]
//        "boost_locale-mt",
//        "boost_program_options",
//        "boost_regex",
//        "boost_serialization",
//        "boost_system",
//        #[cfg(target_os = "linux")]
//        "boost_thread",
//        #[cfg(target_os = "macos")]
//        "boost_thread-mt",
//    ];
//
//    let known_paths = vec![
//        "/usr/lib".to_string(),
//        "/usr/lib/x86_64-linux-gnu".to_string(),
//        "/usr/local/lib".to_string(),
//        format!("{}/brew/lib", dirs::home_dir().unwrap().display()),
//    ];
//
//    for known in &known_paths {
//        println!("cargo:rustc-link-search={}", known);
//    }
//
//    for lib in &boost_libs {
//        println!("cargo:rustc-link-lib={}", lib);
//    }
//}
