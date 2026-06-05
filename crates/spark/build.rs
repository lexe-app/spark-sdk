fn main() {
    // `target_family` is unset on some targets (e.g. SGX), so default it rather
    // than panicking; an absent family is never wasm.
    let target_family = std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default();
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let is_wasm = target_family == "wasm" && target_os == "unknown";

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .build_transport(!is_wasm)
        .compile_protos(
            &[
                "protos/spark/common.proto",
                "protos/spark/multisig.proto",
                "protos/spark/spark.proto",
                "protos/spark/spark_authn.proto",
                "protos/spark/spark_token.proto",
            ],
            &["protos"],
        )
        .unwrap();

    println!("cargo:rerun-if-changed=protos");

    built::write_built_file().expect("Failed to acquire build-time information");
}
