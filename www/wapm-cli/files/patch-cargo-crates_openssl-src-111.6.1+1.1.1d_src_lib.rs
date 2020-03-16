--- cargo-crates/openssl-src-111.6.1+1.1.1d/src/lib.rs.orig	2019-12-02 18:13:32 UTC
+++ cargo-crates/openssl-src-111.6.1+1.1.1d/src/lib.rs
@@ -135,6 +135,7 @@ impl Build {
             // that bypasses basically everything `cc` does, so let's just cop
             // out and say it's linux and hope it works.
             "aarch64-linux-android" => "linux-aarch64",
+            "aarch64-unknown-freebsd" => "BSD-generic64",
             "aarch64-unknown-linux-gnu" => "linux-aarch64",
             "aarch64-unknown-linux-musl" => "linux-aarch64",
             "aarch64-pc-windows-msvc" => "VC-WIN64-ARM",
@@ -144,6 +145,8 @@ impl Build {
             "arm-unknown-linux-gnueabihf" => "linux-armv4",
             "arm-unknown-linux-musleabi" => "linux-armv4",
             "arm-unknown-linux-musleabihf" => "linux-armv4",
+            "armv6-unknown-freebsd" => "BSD-generic32",
+            "armv7-unknown-freebsd" => "BSD-generic32",
             "armv7-unknown-linux-gnueabihf" => "linux-armv4",
             "armv7-unknown-linux-musleabihf" => "linux-armv4",
             "asmjs-unknown-emscripten" => "gcc",
@@ -161,6 +164,7 @@ impl Build {
             "mipsel-unknown-linux-gnu" => "linux-mips32",
             "mipsel-unknown-linux-musl" => "linux-mips32",
             "powerpc-unknown-linux-gnu" => "linux-ppc",
+            "powerpc64-unknown-freebsd" => "BSD-generic64",
             "powerpc64-unknown-linux-gnu" => "linux-ppc64",
             "powerpc64le-unknown-linux-gnu" => "linux-ppc64le",
             "s390x-unknown-linux-gnu" => "linux64-s390x",
