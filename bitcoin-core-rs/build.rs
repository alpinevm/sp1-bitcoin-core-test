use std::env;

fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("vendor/bitcoin/src"); // include path
    let mut b = autocxx_build::Builder::new("src/lib.rs", &[&path]).build()?;

    if env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "riscv32" {
        const DEFAULT_RISCV_GNU_TOOLCHAIN: &str = "/opt/riscv";
        println!("cargo:rerun-if-env-changed=RISCV_GNU_TOOLCHAIN");
    
        let riscv_gnu_toolchain_path = env::var("RISCV_GNU_TOOLCHAIN").unwrap_or_else(|_| {
            println!("cargo:warning=Variable RISCV_GNU_TOOLCHAIN unset. Assuming '{DEFAULT_RISCV_GNU_TOOLCHAIN}'");
            println!("cargo:warning=Please make sure to build riscv toolchain:");
            println!("cargo:warning=  git clone https://github.com/riscv-collab/riscv-gnu-toolchain && cd riscv-gnu-toolchain");
            println!("cargo:warning=  export RISCV_GNU_TOOLCHAIN={DEFAULT_RISCV_GNU_TOOLCHAIN}");
            println!("cargo:warning=  configure --prefix=\"$RISCV_GNU_TOOLCHAIN\" --with-arch=rv32im --with-abi=ilp32");
            println!("cargo:warning=  make -j$(nproc)");
    
            // if unset, try the default and fail eventually
            DEFAULT_RISCV_GNU_TOOLCHAIN.into()
        });
    
        b.compiler("clang")
            .no_default_flags(true)
            .flag(&format!("--sysroot={riscv_gnu_toolchain_path}/riscv32-unknown-elf"))
            .flag(&format!("--gcc-toolchain={riscv_gnu_toolchain_path}"))
            .flag("--target=riscv32-unknown-none-elf")
            .flag("-march=rv32im")
            .flag("-mabi=ilp32")
            .flag("-mcmodel=medany")
            .flag("-Os")
            .flag("-fdata-sections")
            .flag("-ffunction-sections")
            .flag("-flto")
            .target("riscv32im-unknown-none-elf");
    }

    b.std("c++20")
        .file("vendor/bitcoin/src/crypto/sha256.cpp") // Add the implementation file
        .compile("bitcoin-core-rs-bindings");

    println!("cargo:rerun-if-changed=src/lib.rs");
    Ok(())
}
