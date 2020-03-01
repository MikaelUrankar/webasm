XXX MINSIGSTKSZ arch dependant?

--- lucet-runtime/lucet-runtime-internals/src/alloc/mod.rs.orig	2020-03-01 21:33:02.601642000 +0100
+++ lucet-runtime/lucet-runtime-internals/src/alloc/mod.rs	2020-03-01 21:35:40.220299000 +0100
@@ -419,11 +419,15 @@ pub struct Limits {
     pub signal_stack_size: usize,
 }
 
+// this constant isn't exported by `libc` on FreeBSD
+#[cfg(target_os = "freebsd")]
+pub const MINSIGSTKSZ: usize = 4 * 512;
+
 // this constant isn't exported by `libc` on Mac
 #[cfg(target_os = "macos")]
 pub const MINSIGSTKSZ: usize = 32 * 1024;
 
-#[cfg(not(target_os = "macos"))]
+#[cfg(not(any(target_os = "macos", target_os = "freebsd")))]
 pub const MINSIGSTKSZ: usize = libc::MINSIGSTKSZ;
 
 // on Linux, `SIGSTKSZ` is too small for the signal handler when compiled in debug mode
