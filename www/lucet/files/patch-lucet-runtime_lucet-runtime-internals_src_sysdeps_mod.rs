--- lucet-runtime/lucet-runtime-internals/src/sysdeps/mod.rs.orig	2019-12-16 19:11:25 UTC
+++ lucet-runtime/lucet-runtime-internals/src/sysdeps/mod.rs
@@ -4,8 +4,14 @@ mod macos;
 #[cfg(target_os = "linux")]
 mod linux;
 
+#[cfg(target_os = "freebsd")]
+mod freebsd;
+
 #[cfg(target_os = "macos")]
 pub use macos::*;
 
 #[cfg(target_os = "linux")]
 pub use linux::*;
+
+#[cfg(target_os = "freebsd")]
+pub use freebsd::*;
