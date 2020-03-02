--- cargo-crates/llvm-sys-80.1.2/build.rs.orig	2020-03-02 13:36:27 UTC
+++ cargo-crates/llvm-sys-80.1.2/build.rs
@@ -59,6 +59,7 @@ lazy_static! {
             "llvm-config".into(),
             format!("llvm-config-{}", CRATE_VERSION.major),
             format!("llvm-config-{}.{}", CRATE_VERSION.major, CRATE_VERSION.minor),
+            format!("llvm-config{}{}", CRATE_VERSION.major, CRATE_VERSION.minor),
         ]
     };
 
