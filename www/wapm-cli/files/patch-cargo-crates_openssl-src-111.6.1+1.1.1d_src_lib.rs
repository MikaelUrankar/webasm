--- cargo-crates/openssl-src-111.6.1+1.1.1d/src/lib.rs.orig	2019-12-02 18:13:32 UTC
+++ cargo-crates/openssl-src-111.6.1+1.1.1d/src/lib.rs
@@ -51,10 +51,11 @@ impl Build {
     }
 
     fn cmd_make(&self) -> Command {
-        match &self.host.as_ref().expect("HOST dir not set")[..] {
-            "x86_64-unknown-dragonfly" => Command::new("gmake"),
-            "x86_64-unknown-freebsd" => Command::new("gmake"),
-            _ => Command::new("make"),
+        let host = &self.host.as_ref().expect("HOST dir not set")[..];
+        if host.contains("dragonfly") || host.contains("freebsd") {
+            Command::new("gmake")
+        } else {
+            Command::new("make")
         }
     }
 
@@ -135,6 +136,7 @@ impl Build {
             // that bypasses basically everything `cc` does, so let's just cop
             // out and say it's linux and hope it works.
             "aarch64-linux-android" => "linux-aarch64",
+            "aarch64-unknown-freebsd" => "BSD-generic64",
             "aarch64-unknown-linux-gnu" => "linux-aarch64",
             "aarch64-unknown-linux-musl" => "linux-aarch64",
             "aarch64-pc-windows-msvc" => "VC-WIN64-ARM",
@@ -144,6 +146,8 @@ impl Build {
             "arm-unknown-linux-gnueabihf" => "linux-armv4",
             "arm-unknown-linux-musleabi" => "linux-armv4",
             "arm-unknown-linux-musleabihf" => "linux-armv4",
+            "armv6-unknown-freebsd" => "BSD-generic32",
+            "armv7-unknown-freebsd" => "BSD-generic32",
             "armv7-unknown-linux-gnueabihf" => "linux-armv4",
             "armv7-unknown-linux-musleabihf" => "linux-armv4",
             "asmjs-unknown-emscripten" => "gcc",
@@ -161,6 +165,7 @@ impl Build {
             "mipsel-unknown-linux-gnu" => "linux-mips32",
             "mipsel-unknown-linux-musl" => "linux-mips32",
             "powerpc-unknown-linux-gnu" => "linux-ppc",
+            "powerpc64-unknown-freebsd" => "BSD-generic64",
             "powerpc64-unknown-linux-gnu" => "linux-ppc64",
             "powerpc64le-unknown-linux-gnu" => "linux-ppc64le",
             "s390x-unknown-linux-gnu" => "linux64-s390x",
