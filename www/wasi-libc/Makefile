# $FreeBSD$

PORTNAME=	wasi-libc
DISTVERSION=	g20191220
CATEGORIES=	www
MASTER_SITES=	GH

MAINTAINER=	mikael.urankar@gmail.com
COMMENT=	WASI libc implementation for WebAssembly

# LICENSE=	NONE
BUILD_DEPENDS=	clang90:devel/llvm90

USES=	gmake

USE_GITHUB=	yes
GH_ACCOUNT=	CraneStation
GH_TAGNAME=	a280fea

PATCH_ARGS=	-p1

# XXX INSTALL_DIR
INSTALL_DIR=	${STAGEDIR}${LOCALBASE}/share/wasi-sysroot
MAKE_ENV+=	WASM_CC=clang90 WASM_NM=llvm-nm90 WASM_AR=llvm-ar90

ALL_TARGET=	finish

post-patch:
	${RM} ${WRKSRC}/Makefile.orig \
		${WRKSRC}/sysroot/include/stdarg.h.orig \
		${WRKSRC}/sysroot/include/stddef.h.orig \
		${WRKSRC}/basics/include/stddef.h.orig \
		${WRKSRC}/basics/include/stdarg.h.orig \
		${WRKSRC}/expected/wasm32-wasi/include-all.c.orig

# the Makefile sucks, it rebuilds wasi-libc in the install target
do-install:
	${MKDIR} ${INSTALL_DIR}
	(cd ${WRKSRC}/sysroot && ${COPYTREE_SHARE} "lib share include" ${INSTALL_DIR})

.include <bsd.port.mk>
