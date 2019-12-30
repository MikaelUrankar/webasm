--- lucet-runtime/lucet-runtime-internals/src/sysdeps/freebsd.rs.orig	2019-12-16 19:10:58 UTC
+++ lucet-runtime/lucet-runtime-internals/src/sysdeps/freebsd.rs
@@ -0,0 +1,116 @@
+use libc::{c_long, c_int, c_void, sigset_t, size_t};
+
+#[derive(Clone, Copy)]
+#[repr(C)]
+pub struct ucontext_t {
+    uc_sigmask: sigset_t,
+    uc_mcontext: mcontext_t,
+    uc_link: *mut ucontext_t,
+    uc_stack: stack_t,
+    uc_flags: c_int,
+    __spare__: [c_int; 4],
+}
+
+#[derive(Clone, Copy)]
+#[repr(C)]
+pub struct stack_t {
+    ss_sp: *mut c_void,
+    ss_size: size_t,
+    ss_flags: c_int,
+}
+
+#[derive(Clone, Copy)]
+#[repr(C)]
+pub struct mcontext_t {
+    mc_onstack: u64,
+    mc_rdi: u64,
+    mc_rsi: u64,
+    mc_rdx: u64,
+    mc_rcx: u64,
+    mc_r8: u64,
+    mc_r9: u64,
+    mc_rax: u64,
+    mc_rbx: u64,
+    mc_rbp: u64,
+    mc_r10: u64,
+    mc_r11: u64,
+    mc_r12: u64,
+    mc_r13: u64,
+    mc_r14: u64,
+    mc_r15: u64,
+    mc_trapno: u32,
+    mc_fs: u16,
+    mc_gs: u16,
+    mc_addr: u64,
+    mc_flags: u32,
+    mc_es: u16,
+    mc_ds: u16,
+    mc_err: u64,
+    mc_rip: u64,
+    mc_cs: u64,
+    mc_rflags: u64,
+    mc_rsp: u64,
+    mc_ss: u64,
+    mc_len: c_long,
+
+    mc_fpformat: c_long,
+    mc_ownedfp: c_long,
+    mc_fpstate: [c_long; 64],
+
+    mc_fsbase: u64,
+    mc_gsbase: u64,
+
+    mc_xfpustate: u64,
+    mc_xfpustate_len: u64,
+
+    mc_spare: [c_long; 4],
+}
+
+#[derive(Clone, Copy, Debug)]
+pub struct UContextPtr(*const ucontext_t);
+
+impl UContextPtr {
+    #[inline]
+    pub fn new(ptr: *const c_void) -> Self {
+        assert!(!ptr.is_null(), "non-null context");
+        UContextPtr(ptr as *const ucontext_t)
+    }
+
+    #[inline]
+    pub fn get_ip(self) -> *const c_void {
+        let mcontext = &unsafe { *(self.0) }.uc_mcontext;
+        mcontext.mc_rip as *const _
+    }
+}
+
+#[repr(C)]
+#[derive(Clone, Copy)]
+pub struct UContext {
+    context: ucontext_t,
+}
+
+impl UContext {
+    #[inline]
+    pub fn new(ptr: *const c_void) -> Self {
+        UContext {
+            context: *unsafe {
+                (ptr as *const ucontext_t)
+                    .as_ref()
+                    .expect("non-null context")
+            },
+        }
+    }
+
+    pub fn as_ptr(&mut self) -> UContextPtr {
+        UContextPtr::new(&self.context as *const _ as *const _)
+    }
+}
+
+impl Into<UContext> for UContextPtr {
+    #[inline]
+    fn into(self) -> UContext {
+        UContext {
+            context: unsafe { *(self.0) },
+        }
+    }
+}
