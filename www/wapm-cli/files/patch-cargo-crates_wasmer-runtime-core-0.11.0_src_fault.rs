--- cargo-crates/wasmer-runtime-core-0.11.0/src/fault.rs.orig	2019-11-22 20:15:44.000000000 +0100
+++ cargo-crates/wasmer-runtime-core-0.11.0/src/fault.rs	2019-12-10 19:56:33.488102000 +0100
@@ -746,3 +746,198 @@ pub unsafe fn get_fault_info(siginfo: *const c_void, u
         known_registers,
     }
 }
+
+#[cfg(all(target_os = "freebsd", target_arch = "x86_64"))]
+/// Get fault info from siginfo and ucontext.
+pub unsafe fn get_fault_info(siginfo: *const c_void, ucontext: *mut c_void) -> FaultInfo {
+    #[allow(dead_code)]
+    #[allow(non_camel_case_types)]
+    #[repr(C)]
+    pub struct ucontext_t {
+        pub uc_sigmask: libc::sigset_t,
+        pub uc_mcontext: mcontext_t,
+        pub uc_link: *mut ucontext_t,
+        pub uc_stack: libc::stack_t,
+        pub uc_flags: i32,
+        pub __spare__: [i32; 4],
+    }
+
+    #[allow(dead_code)]
+    #[allow(non_camel_case_types)]
+    #[repr(C)]
+    pub struct mcontext_t {
+        mc_onstack: u64,
+        mc_rdi: u64,
+        mc_rsi: u64,
+        mc_rdx: u64,
+        mc_rcx: u64,
+        mc_r8: u64,
+        mc_r9: u64,
+        mc_rax: u64,
+        mc_rbx: u64,
+        mc_rbp: u64,
+        mc_r10: u64,
+        mc_r11: u64,
+        mc_r12: u64,
+        mc_r13: u64,
+        mc_r14: u64,
+        mc_r15: u64,
+        mc_trapno: u32,
+        mc_fs: u16,
+        mc_gs: u16,
+        mc_addr: u64,
+        mc_flags: u32,
+        mc_es: u16,
+        mc_ds: u16,
+        mc_err: u64,
+        mc_rip: u64,
+        mc_cs: u64,
+        mc_rflags: u64,
+        mc_rsp: u64,
+        mc_ss: u64,
+        mc_len: i64,
+
+        mc_fpformat: i64,
+        mc_ownedfp: i64,
+        mc_fpstate: [i64; 32],
+        mc_fpstate2: [i64; 32],
+
+        mc_fsbase: u64,
+        mc_gsbase: u64,
+
+        mc_xfpustate: u64,
+        mc_xfpustate_len: u64,
+
+        mc_spare: [i64; 4],
+    }
+
+    let siginfo = siginfo as *const siginfo_t;
+    let si_addr = (*siginfo).si_addr;
+
+    let ucontext = ucontext as *mut ucontext_t;
+    let gregs = &mut (*ucontext).uc_mcontext;
+
+    let mut known_registers: [Option<u64>; 32] = [None; 32];
+    known_registers[X64Register::GPR(GPR::R15).to_index().0] = Some(gregs.mc_r15);
+    known_registers[X64Register::GPR(GPR::R14).to_index().0] = Some(gregs.mc_r14);
+    known_registers[X64Register::GPR(GPR::R13).to_index().0] = Some(gregs.mc_r13);
+    known_registers[X64Register::GPR(GPR::R12).to_index().0] = Some(gregs.mc_r12);
+    known_registers[X64Register::GPR(GPR::R11).to_index().0] = Some(gregs.mc_r11);
+    known_registers[X64Register::GPR(GPR::R10).to_index().0] = Some(gregs.mc_r10);
+    known_registers[X64Register::GPR(GPR::R9).to_index().0] = Some(gregs.mc_r9);
+    known_registers[X64Register::GPR(GPR::R8).to_index().0] = Some(gregs.mc_r8);
+    known_registers[X64Register::GPR(GPR::RSI).to_index().0] = Some(gregs.mc_rsi);
+    known_registers[X64Register::GPR(GPR::RDI).to_index().0] = Some(gregs.mc_rdi);
+    known_registers[X64Register::GPR(GPR::RDX).to_index().0] = Some(gregs.mc_rdx);
+    known_registers[X64Register::GPR(GPR::RCX).to_index().0] = Some(gregs.mc_rcx);
+    known_registers[X64Register::GPR(GPR::RBX).to_index().0] = Some(gregs.mc_rbx);
+    known_registers[X64Register::GPR(GPR::RAX).to_index().0] = Some(gregs.mc_rax);
+
+    known_registers[X64Register::GPR(GPR::RBP).to_index().0] = Some(gregs.mc_rbp);
+    known_registers[X64Register::GPR(GPR::RSP).to_index().0] = Some(gregs.mc_rsp);
+
+//    known_registers[X64Register::XMM(XMM::XMM0).to_index().0] = Some(fs.xmm[0][0]);
+//    known_registers[X64Register::XMM(XMM::XMM1).to_index().0] = Some(fs.xmm[1][0]);
+//    known_registers[X64Register::XMM(XMM::XMM2).to_index().0] = Some(fs.xmm[2][0]);
+//    known_registers[X64Register::XMM(XMM::XMM3).to_index().0] = Some(fs.xmm[3][0]);
+//    known_registers[X64Register::XMM(XMM::XMM4).to_index().0] = Some(fs.xmm[4][0]);
+//    known_registers[X64Register::XMM(XMM::XMM5).to_index().0] = Some(fs.xmm[5][0]);
+//    known_registers[X64Register::XMM(XMM::XMM6).to_index().0] = Some(fs.xmm[6][0]);
+//    known_registers[X64Register::XMM(XMM::XMM7).to_index().0] = Some(fs.xmm[7][0]);
+//    known_registers[X64Register::XMM(XMM::XMM8).to_index().0] = Some(fs.xmm[8][0]);
+//    known_registers[X64Register::XMM(XMM::XMM9).to_index().0] = Some(fs.xmm[9][0]);
+//    known_registers[X64Register::XMM(XMM::XMM10).to_index().0] = Some(fs.xmm[10][0]);
+//    known_registers[X64Register::XMM(XMM::XMM11).to_index().0] = Some(fs.xmm[11][0]);
+//    known_registers[X64Register::XMM(XMM::XMM12).to_index().0] = Some(fs.xmm[12][0]);
+//    known_registers[X64Register::XMM(XMM::XMM13).to_index().0] = Some(fs.xmm[13][0]);
+//    known_registers[X64Register::XMM(XMM::XMM14).to_index().0] = Some(fs.xmm[14][0]);
+//    known_registers[X64Register::XMM(XMM::XMM15).to_index().0] = Some(fs.xmm[15][0]);
+
+    FaultInfo {
+        faulting_addr: si_addr,
+        ip: std::mem::transmute::<&mut u64, &'static Cell<usize>>(&mut (*ucontext).uc_mcontext.mc_rip),
+        known_registers,
+    }
+}
+
+#[cfg(all(target_os = "freebsd", target_arch = "aarch64"))]
+/// Get fault info from siginfo and ucontext.
+pub unsafe fn get_fault_info(siginfo: *const c_void, ucontext: *mut c_void) -> FaultInfo {
+    #[allow(dead_code)]
+    #[allow(non_camel_case_types)]
+    #[repr(C)]
+    pub struct ucontext_t {
+        pub uc_sigmask: libc::sigset_t,
+        pub uc_mcontext: mcontext_t,
+        pub uc_link: *mut ucontext_t,
+        pub uc_stack: libc::stack_t,
+        pub uc_flags: i32,
+        pub __spare__: [i32; 4],
+    }
+
+    #[allow(dead_code)]
+    #[allow(non_camel_case_types)]
+    #[repr(C)]
+    pub struct gpregs {
+        pub gp_x: [u64; 30],
+        pub gp_lr: u64,
+        pub gp_sp: u64,
+        pub gp_elr: u64,
+        pub gp_spsr: u64,
+        pub gp_pad: i32,
+    };
+
+    #[allow(dead_code)]
+    #[allow(non_camel_case_types)]
+    #[repr(C)]
+    pub struct fpregs {
+        pub fp_q: [u128; 32],
+        pub fp_sr: u32,
+        pub fp_cr: u32,
+        pub fp_flags: i32,
+        pub fp_pad: i32,
+    };
+
+    #[allow(dead_code)]
+    #[allow(non_camel_case_types)]
+    #[repr(C)]
+    pub struct mcontext_t {
+        pub mc_gpregs: gpregs,
+        pub mc_fpregs: fpregs,
+        pub mc_flags: i32,
+        pub mc_pad: i32,
+        pub mc_spare: [u64; 8],
+    }
+
+    let siginfo = siginfo as *const siginfo_t;
+    let si_addr = (*siginfo).si_addr;
+
+    let ucontext = ucontext as *mut ucontext_t;
+    let gregs = &(*ucontext).uc_mcontext.mc_gpregs;
+
+    let mut known_registers: [Option<u64>; 32] = [None; 32];
+
+    known_registers[X64Register::GPR(GPR::R15).to_index().0] = Some(gregs.gp_x[15] as _);
+    known_registers[X64Register::GPR(GPR::R14).to_index().0] = Some(gregs.gp_x[14] as _);
+    known_registers[X64Register::GPR(GPR::R13).to_index().0] = Some(gregs.gp_x[13] as _);
+    known_registers[X64Register::GPR(GPR::R12).to_index().0] = Some(gregs.gp_x[12] as _);
+    known_registers[X64Register::GPR(GPR::R11).to_index().0] = Some(gregs.gp_x[11] as _);
+    known_registers[X64Register::GPR(GPR::R10).to_index().0] = Some(gregs.gp_x[10] as _);
+    known_registers[X64Register::GPR(GPR::R9).to_index().0] = Some(gregs.gp_x[9] as _);
+    known_registers[X64Register::GPR(GPR::R8).to_index().0] = Some(gregs.gp_x[8] as _);
+    known_registers[X64Register::GPR(GPR::RSI).to_index().0] = Some(gregs.gp_x[6] as _);
+    known_registers[X64Register::GPR(GPR::RDI).to_index().0] = Some(gregs.gp_x[7] as _);
+    known_registers[X64Register::GPR(GPR::RDX).to_index().0] = Some(gregs.gp_x[2] as _);
+    known_registers[X64Register::GPR(GPR::RCX).to_index().0] = Some(gregs.gp_x[1] as _);
+    known_registers[X64Register::GPR(GPR::RBX).to_index().0] = Some(gregs.gp_x[3] as _);
+    known_registers[X64Register::GPR(GPR::RAX).to_index().0] = Some(gregs.gp_x[0] as _);
+
+    known_registers[X64Register::GPR(GPR::RBP).to_index().0] = Some(gregs.gp_x[5] as _);
+    known_registers[X64Register::GPR(GPR::RSP).to_index().0] = Some(gregs.gp_x[28] as _);
+
+    FaultInfo {
+        faulting_addr: si_addr as usize as _,
+        ip: std::mem::transmute::<&mut u64, &'static Cell<usize>>(&mut (*ucontext).uc_mcontext.mc_gpregs.gp_elr),
+        known_registers,
+    }
+}
