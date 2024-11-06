use std::env;

fn main() {
    let mut base_config = cc::Build::new();
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

        base_config
            .compiler("clang++")
            .no_default_flags(true)
            .flag(&format!(
                "--sysroot={riscv_gnu_toolchain_path}/riscv32-unknown-elf"
            ))
            .flag(&format!("--gcc-toolchain={riscv_gnu_toolchain_path}"))
            .flag("--target=riscv32-unknown-none-elf")
            .flag("-march=rv32im")
            .flag("-mabi=ilp32")
            .flag("-mcmodel=medany")
            .flag("-Os")
            .flag("-fdata-sections")
            .flag("-ffunction-sections")
            .flag("-flto")
            .flag("-fno-exceptions")
            .flag("-fno-rtti")
            .flag("-fno-threadsafe-statics")
            .target("riscv32im-unknown-none-elf");
    }

    base_config
        .cpp(true)
        .flag("-std=c++20")
        .flag("-stdlib=libc++")
        .include("src/native/vendor/bitcoin/src")
        .file("src/native/vendor/bitcoin/src/crypto/sha256.cpp")
        .file("src/native/sha256_wrapper.cpp")
        .compile("sha256");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/native/sha256_wrapper.cpp");
}
