--- cargo-crates/terminal_size-0.1.10/src/unix.rs.orig	2020-03-01 21:06:48 UTC
+++ cargo-crates/terminal_size-0.1.10/src/unix.rs
@@ -28,7 +28,7 @@ pub fn terminal_size_using_fd(fd: RawFd) -> Option<(Wi
         ws_ypixel: 0,
     };
 
-    if unsafe { ioctl(fd, TIOCGWINSZ, &mut winsize) } == -1 {
+    if unsafe { ioctl(fd, TIOCGWINSZ.into(), &mut winsize) } == -1 {
         return None;
     }
 
