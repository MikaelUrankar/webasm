--- lib/spectests/tests/spectest.rs.orig	2020-03-02 15:30:36.839428000 +0100
+++ lib/spectests/tests/spectest.rs	2020-03-02 15:28:45.705999000 +0100
@@ -112,6 +112,11 @@ mod tests {
         "windows"
     }
 
+    #[cfg(target_os = "freebsd")]
+    fn get_target_os() -> &'static str {
+        "freebsd"
+    }
+
     fn get_target_arch() -> &'static str {
         if cfg!(target_arch = "x86_64") {
             "x86_64"
