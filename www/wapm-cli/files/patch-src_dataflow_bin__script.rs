--- src/dataflow/bin_script.rs.orig	2019-12-14 14:15:43.322122000 +0100
+++ src/dataflow/bin_script.rs	2019-12-14 14:16:02.926352000 +0100
@@ -15,7 +15,7 @@ pub enum Error {
 
 #[cfg(not(target_os = "windows"))]
 pub fn save_bin_script<P: AsRef<Path>>(directory: P, command_name: String) -> Result<(), Error> {
-    let data = format!("#!/bin/bash\nwapm run {} \"$@\"\n", command_name);
+    let data = format!("#!/bin/sh\nwapm run {} \"$@\"\n", command_name);
     save(data, directory, command_name)
 }
 
