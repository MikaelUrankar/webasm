# $FreeBSD$

PORTNAME=	lucet
DISTVERSION=	0.4.1
CATEGORIES=	www
MASTER_SITES=	GH

MAINTAINER=	mikael.urankar@gmail.com
COMMENT=	Lucet, the Sandboxing WebAssembly Compiler

LICENSE=	APACHE20

LIB_DEPENDS=	libhwloc.so:devel/hwloc
RUN_DEPENDS=	cargo:lang/${RUST_DEFAULT}

USES=		cargo

USE_LDCONFIG=	yes

USE_GITHUB=	yes
GH_ACCOUNT=	bytecodealliance
GH_TAGNAME=	6b9c559

GH_TUPLE=	bytecodealliance:cranelift:v0.51.0:cranelift/cranelift \
		bytecodealliance:sightglass:da53460:sightglass/sightglass \
		webassembly:spec:v1.1:spec/lucet-spectest/spec \
		webassembly:WASI:db73914:WASI/wasi

CARGO_CRATES=	adler32-1.0.4 \
		aho-corasick-0.7.6 \
		ansi_term-0.11.0 \
		anyhow-1.0.23 \
		approx-0.1.1 \
		arrayvec-0.4.12 \
		atty-0.2.13 \
		autocfg-0.1.7 \
		backtrace-0.3.40 \
		backtrace-sys-0.1.32 \
		base64-0.10.1 \
		bencher-0.1.5 \
		bimap-0.2.0 \
		bincode-1.2.0 \
		bindgen-0.51.1 \
		bitflags-0.7.0 \
		bitflags-1.2.1 \
		block-buffer-0.7.3 \
		block-padding-0.1.4 \
		bstr-0.2.8 \
		byte-tools-0.3.1 \
		byteorder-1.3.2 \
		c2-chacha-0.2.3 \
		cast-0.2.2 \
		cc-1.0.46 \
		cexpr-0.3.5 \
		cfg-if-0.1.10 \
		cgmath-0.16.1 \
		clang-sys-0.28.1 \
		clap-2.33.0 \
		cloudabi-0.0.3 \
		cmake-0.1.42 \
		colored-1.8.0 \
		core_affinity-0.5.9 \
		cpu-time-1.0.0 \
		crc32fast-1.2.0 \
		criterion-0.3.0 \
		criterion-plot-0.4.0 \
		crossbeam-deque-0.7.1 \
		crossbeam-epoch-0.7.2 \
		crossbeam-queue-0.1.2 \
		crossbeam-utils-0.6.6 \
		crypto-mac-0.7.0 \
		csv-1.1.1 \
		csv-core-0.1.6 \
		cvt-0.1.1 \
		derivative-1.0.3 \
		digest-0.8.1 \
		either-1.5.3 \
		env_logger-0.6.2 \
		errno-0.2.4 \
		errno-dragonfly-0.1.1 \
		faerie-0.12.0 \
		failure-0.1.6 \
		failure_derive-0.1.6 \
		fake-simd-0.1.2 \
		filetime-0.2.7 \
		flate2-1.0.12 \
		fuchsia-cprng-0.1.1 \
		gcc-0.3.55 \
		generic-array-0.12.3 \
		getrandom-0.1.3 \
		glob-0.2.11 \
		glob-0.3.0 \
		goblin-0.0.24 \
		goblin-0.1.1 \
		heck-0.3.1 \
		hermit-abi-0.1.3 \
		hmac-0.7.1 \
		human-size-0.4.0 \
		humantime-1.3.0 \
		hwloc-0.5.0 \
		indexmap-1.3.0 \
		itertools-0.8.1 \
		itoa-0.4.4 \
		kernel32-sys-0.2.2 \
		lazy_static-1.4.0 \
		leb128-0.2.4 \
		libc-0.2.65 \
		libloading-0.5.2 \
		log-0.4.8 \
		memchr-2.2.1 \
		memoffset-0.5.2 \
		minisign-0.5.11 \
		miniz_oxide-0.3.5 \
		nix-0.15.0 \
		nodrop-0.1.14 \
		nom-4.2.3 \
		num-0.1.42 \
		num-0.2.0 \
		num-complex-0.2.3 \
		num-derive-0.3.0 \
		num-integer-0.1.41 \
		num-iter-0.1.39 \
		num-rational-0.2.2 \
		num-traits-0.1.43 \
		num-traits-0.2.8 \
		num_cpus-1.11.0 \
		object-0.14.1 \
		opaque-debug-0.2.3 \
		packed_struct-0.3.0 \
		packed_struct_codegen-0.3.0 \
		parity-wasm-0.40.3 \
		parity-wasm-0.41.0 \
		pbkdf2-0.3.0 \
		peeking_take_while-0.1.2 \
		pkg-config-0.3.17 \
		plain-0.2.3 \
		ppv-lite86-0.2.6 \
		precision-0.1.9 \
		printtable-0.1.0 \
		proc-macro-error-0.2.6 \
		proc-macro2-0.4.30 \
		proc-macro2-1.0.6 \
		progress-0.2.0 \
		quick-error-1.2.2 \
		quote-0.3.15 \
		quote-0.6.13 \
		quote-1.0.2 \
		rand-0.4.6 \
		rand-0.6.5 \
		rand-0.7.2 \
		rand_chacha-0.1.1 \
		rand_chacha-0.2.1 \
		rand_core-0.3.1 \
		rand_core-0.4.2 \
		rand_core-0.5.1 \
		rand_hc-0.1.0 \
		rand_hc-0.2.0 \
		rand_isaac-0.1.1 \
		rand_jitter-0.1.4 \
		rand_os-0.1.3 \
		rand_os-0.2.2 \
		rand_pcg-0.1.2 \
		rand_xorshift-0.1.1 \
		rand_xoshiro-0.3.1 \
		raw-cpuid-6.1.0 \
		raw-cpuid-7.0.3 \
		rayon-1.2.0 \
		rayon-core-1.6.0 \
		rdrand-0.4.0 \
		redox_syscall-0.1.56 \
		regex-1.3.1 \
		regex-automata-0.1.8 \
		regex-syntax-0.6.12 \
		remove_dir_all-0.5.2 \
		rgb-0.8.14 \
		rpassword-3.0.2 \
		rustc-demangle-0.1.16 \
		rustc-hash-1.0.1 \
		rustc_version-0.2.3 \
		ryu-1.0.2 \
		same-file-1.0.5 \
		scopeguard-1.0.0 \
		scroll-0.9.2 \
		scroll-0.10.1 \
		scroll_derive-0.9.5 \
		scroll_derive-0.10.1 \
		scrypt-0.2.0 \
		semver-0.9.0 \
		semver-parser-0.7.0 \
		serde-1.0.102 \
		serde-big-array-0.2.0 \
		serde_derive-1.0.102 \
		serde_json-1.0.41 \
		sha2-0.8.0 \
		shlex-0.1.1 \
		siphasher-0.3.1 \
		smallvec-1.0.0 \
		string-interner-0.7.1 \
		strsim-0.8.0 \
		structopt-0.3.3 \
		structopt-derive-0.3.3 \
		subtle-1.0.0 \
		syn-0.11.11 \
		syn-0.15.44 \
		syn-1.0.7 \
		synom-0.11.3 \
		synstructure-0.12.1 \
		target-lexicon-0.8.1 \
		target-lexicon-0.9.0 \
		tempfile-3.1.0 \
		termcolor-1.0.5 \
		terminal_size-0.1.8 \
		textwrap-0.11.0 \
		thiserror-1.0.6 \
		thiserror-impl-1.0.6 \
		thread_local-0.3.6 \
		tinytemplate-1.0.2 \
		toml-0.5.5 \
		typenum-1.11.2 \
		unicode-segmentation-1.5.0 \
		unicode-width-0.1.6 \
		unicode-xid-0.0.4 \
		unicode-xid-0.1.0 \
		unicode-xid-0.2.0 \
		uuid-0.7.4 \
		vec_map-0.8.1 \
		version_check-0.1.5 \
		void-1.0.2 \
		wabt-0.9.2 \
		wabt-sys-0.7.0 \
		wait-timeout-0.2.0 \
		walkdir-2.2.9 \
		wasi-common-0.7.0 \
		wasi-common-cbindgen-0.7.0 \
		wasmonkey-0.1.9 \
		wasmparser-0.39.2 \
		wast-3.0.4 \
		which-3.1.0 \
		wig-0.7.0 \
		winapi-0.2.8 \
		winapi-0.3.8 \
		winapi-build-0.1.1 \
		winapi-i686-pc-windows-gnu-0.4.0 \
		winapi-util-0.1.2 \
		winapi-x86_64-pc-windows-gnu-0.4.0 \
		wincolor-1.0.2 \
		winconsole-0.10.0 \
		winx-0.7.0 \
		witx-0.4.0 \
		xfailure-0.1.0

do-install:
	${INSTALL_LIB} ${CARGO_TARGET_DIR}/release/liblucet*.so ${STAGEDIR}/${LOCALBASE}/lib

.for f in lucet-objdump lucet-validate lucet-wasi sightglass witx lucetc
	${INSTALL_PROGRAM} ${CARGO_TARGET_DIR}/release/${f} ${STAGEDIR}/${LOCALBASE}/bin
.endfor

.include <bsd.port.mk>