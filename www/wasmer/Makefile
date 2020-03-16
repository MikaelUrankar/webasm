# $FreeBSD$

# TODO enable/optionify other backends?
# backend-singlepass requires rust-nightly, loader-kernel requires backend-singlepass
# Add a debug build? cargo build --release --features backend-cranelift,backend-singlepass,debug,trace
# run tests: see ${WRKSRC}/Makefile

PORTNAME=	wasmer
DISTVERSION=	0.16.2
CATEGORIES=	www

MAINTAINER=	mikael.urankar@gmail.com
COMMENT=	Universal WebAssembly Runtime supporting WASI and Emscripten

LICENSE=	MIT

ONLY_FOR_ARCH=	aarch64 amd64
ONLY_FOR_ARCHS_REASON=	not ported to other architectures

USES=		cargo

USE_GITHUB=	yes
GH_ACCOUNT=	wasmerio
GH_TUPLE=	TheDan64:inkwell:af4cf4efbb27cdea8a54175ffc18ffd91964618c:inkwell

# needed for backend-llvm : ld: error: unable to find library -lffi
RUSTFLAGS+=	-L${LOCALBASE}/lib

CARGO_USE_GITHUB=	yes
CARGO_BUILD=	yes
CARGO_INSTALL=	yes
CARGO_FEATURES=	backend-cranelift,backend-llvm,experimental-io-devices,log/release_max_level_off

PLIST_FILES=	bin/kwasmd bin/wasmer

CARGO_CRATES=	aho-corasick-0.7.9 \
		ansi_term-0.11.0 \
		anyhow-1.0.26 \
		arrayref-0.3.6 \
		arrayvec-0.4.12 \
		arrayvec-0.5.1 \
		atty-0.2.14 \
		autocfg-0.1.7 \
		autocfg-1.0.0 \
		bincode-1.2.1 \
		bitflags-1.2.1 \
		blake3-0.1.5 \
		bstr-0.2.11 \
		byteorder-1.3.4 \
		c2-chacha-0.2.3 \
		cast-0.2.3 \
		cbindgen-0.9.1 \
		cc-1.0.50 \
		cfg-if-0.1.10 \
		chrono-0.4.10 \
		clap-2.33.0 \
		cloudabi-0.0.3 \
		cmake-0.1.42 \
		colored-1.9.3 \
		constant_time_eq-0.1.5 \
		cranelift-bforest-0.59.0 \
		cranelift-codegen-0.59.0 \
		cranelift-codegen-meta-0.59.0 \
		cranelift-codegen-shared-0.59.0 \
		cranelift-entity-0.52.0 \
		cranelift-entity-0.59.0 \
		cranelift-native-0.59.0 \
		criterion-0.2.11 \
		criterion-plot-0.3.1 \
		crossbeam-deque-0.7.3 \
		crossbeam-epoch-0.8.2 \
		crossbeam-queue-0.2.1 \
		crossbeam-utils-0.7.2 \
		crypto-mac-0.7.0 \
		csv-1.1.3 \
		csv-core-0.1.10 \
		ctor-0.1.13 \
		digest-0.8.1 \
		dynasm-0.5.2 \
		dynasmrt-0.5.2 \
		either-1.5.3 \
		erased-serde-0.3.10 \
		errno-0.2.4 \
		errno-dragonfly-0.1.1 \
		faerie-0.13.0 \
		fallible-iterator-0.2.0 \
		fern-0.5.9 \
		fuchsia-cprng-0.1.1 \
		gcc-0.3.55 \
		generational-arena-0.2.7 \
		generic-array-0.12.3 \
		getrandom-0.1.14 \
		ghost-0.1.1 \
		gimli-0.20.0 \
		glob-0.2.11 \
		glob-0.3.0 \
		goblin-0.0.24 \
		goblin-0.1.3 \
		heck-0.3.1 \
		hermit-abi-0.1.8 \
		hex-0.4.2 \
		indexmap-1.3.2 \
		inventory-0.1.5 \
		inventory-impl-0.1.5 \
		itertools-0.8.2 \
		itoa-0.4.5 \
		lazy_static-1.4.0 \
		leb128-0.2.4 \
		lexical-core-0.6.2 \
		libc-0.2.67 \
		llvm-sys-80.1.2 \
		lock_api-0.3.3 \
		log-0.4.8 \
		maybe-uninit-2.0.0 \
		md5-0.6.1 \
		memchr-2.3.3 \
		memmap-0.7.0 \
		memoffset-0.5.3 \
		minifb-0.13.0 \
		more-asserts-0.2.1 \
		nix-0.15.0 \
		nodrop-0.1.14 \
		nom-5.1.1 \
		num-0.1.42 \
		num-integer-0.1.42 \
		num-iter-0.1.40 \
		num-traits-0.2.11 \
		num_cpus-1.12.0 \
		once_cell-1.3.1 \
		orbclient-0.3.27 \
		owning_ref-0.4.1 \
		page_size-0.4.2 \
		parking_lot-0.10.0 \
		parking_lot_core-0.7.0 \
		pkg-config-0.3.17 \
		plain-0.2.3 \
		ppv-lite86-0.2.6 \
		proc-macro-error-0.4.11 \
		proc-macro-error-attr-0.4.11 \
		proc-macro2-0.4.30 \
		proc-macro2-1.0.9 \
		quote-0.6.13 \
		quote-1.0.2 \
		rand-0.6.5 \
		rand-0.7.3 \
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
		rand_pcg-0.1.2 \
		rand_xorshift-0.1.1 \
		rand_xoshiro-0.1.0 \
		raw-cpuid-7.0.3 \
		rayon-1.3.0 \
		rayon-core-1.7.0 \
		rdrand-0.4.0 \
		redox_syscall-0.1.56 \
		ref_thread_local-0.0.0 \
		regex-1.3.4 \
		regex-automata-0.1.8 \
		regex-syntax-0.6.15 \
		remove_dir_all-0.5.2 \
		rustc_version-0.2.3 \
		ryu-1.0.2 \
		same-file-1.0.6 \
		scopeguard-1.1.0 \
		scroll-0.9.2 \
		scroll-0.10.1 \
		scroll_derive-0.9.5 \
		scroll_derive-0.10.1 \
		sdl2-0.32.2 \
		sdl2-sys-0.32.6 \
		semver-0.9.0 \
		semver-parser-0.7.0 \
		serde-1.0.104 \
		serde-bench-0.0.7 \
		serde_bytes-0.11.3 \
		serde_derive-1.0.104 \
		serde_json-1.0.48 \
		smallvec-0.6.13 \
		smallvec-1.2.0 \
		stable_deref_trait-1.1.1 \
		static_assertions-0.3.4 \
		string-interner-0.7.1 \
		strsim-0.8.0 \
		structopt-0.3.11 \
		structopt-derive-0.4.4 \
		subtle-1.0.0 \
		syn-0.15.44 \
		syn-1.0.16 \
		syn-mid-0.5.0 \
		target-lexicon-0.9.0 \
		target-lexicon-0.10.0 \
		tempfile-3.1.0 \
		textwrap-0.11.0 \
		thiserror-1.0.11 \
		thiserror-impl-1.0.11 \
		thread_local-1.0.1 \
		time-0.1.42 \
		tinytemplate-1.0.3 \
		toml-0.5.6 \
		typenum-1.11.2 \
		typetag-0.1.4 \
		typetag-impl-0.1.4 \
		unicode-segmentation-1.6.0 \
		unicode-width-0.1.7 \
		unicode-xid-0.1.0 \
		unicode-xid-0.2.0 \
		vec_map-0.8.1 \
		version_check-0.9.1 \
		void-1.0.2 \
		wabt-0.9.2 \
		wabt-sys-0.7.0 \
		walkdir-2.3.1 \
		wasi-0.9.0+wasi-snapshot-preview1 \
		wasm-debug-0.1.0 \
		wasmer-clif-fork-frontend-0.59.0 \
		wasmer-clif-fork-wasm-0.59.0 \
		wasmparser-0.39.3 \
		wasmparser-0.51.4 \
		wast-8.0.0 \
		winapi-0.3.8 \
		winapi-i686-pc-windows-gnu-0.4.0 \
		winapi-util-0.1.3 \
		winapi-x86_64-pc-windows-gnu-0.4.0 \
		x11-dl-2.18.5

post-install:
	${STRIP_CMD} ${STAGEDIR}${LOCALBASE}/bin/kwasmd
	${STRIP_CMD} ${STAGEDIR}${LOCALBASE}/bin/wasmer

.include <bsd.port.mk>
