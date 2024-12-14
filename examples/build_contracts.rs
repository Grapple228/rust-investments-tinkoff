use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir("src/api/v1")
        .compile_protos(
            &[
                "common.proto",
                "instruments.proto",
                "marketdata.proto",
                "operations.proto",
                "orders.proto",
                "sandbox.proto",
                "signals.proto",
                "stoporders.proto",
                "users.proto",
            ],
            &["investAPI/src/docs/contracts/"],
        )?;

    fs::rename(
        "src/api/v1/tinkoff.public.invest.api.contract.v1.rs",
        "src/api/v1/protos.rs",
    )?;

    Ok(())
}
