fn main() {
    // libgit2-sys 0.16.2 (bundling libgit2 1.7.2) compiles libgit2's Windows
    // backends — rand.c, sysdir.c and fs_path.c — which call CryptGenRandom,
    // the Reg* registry APIs and the token/SID security APIs. Those all live in
    // advapi32, but libgit2-sys's build script never emits a link directive for
    // it (it only links winhttp, rpcrt4, ole32, crypt32 and secur32). On the
    // MSVC toolchain that leaves the final link with unresolved __imp_Crypt*,
    // __imp_Reg* and __imp_*Sid externals. Supply the missing dependency here,
    // but only when we actually pull in libgit2 (the `git2` optional dep, which
    // the `libgit` feature turns on).
    let target_windows = std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows");
    let libgit2_in_build = std::env::var_os("CARGO_FEATURE_GIT2").is_some();
    if target_windows && libgit2_in_build {
        println!("cargo:rustc-link-lib=advapi32");
    }
}
