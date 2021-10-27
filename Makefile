CC=cargo
CC_FLAG=--release
BIN=rust-simple-webserver
COMPRESS=upx
COMPRESS_FLAGS=--best --lzma
BUILD_LOCALE=./target/release
FINAL_BIN=rust-simple-webserver

make:
	rm -rf ./target/
	${CC} build ${CC_FLAG}
	strip ${BUILD_LOCALE}/${FINAL_BIN}
	${COMPRESS} ${COMPRESS_FLAGS} ${BUILD_LOCALE}/${FINAL_BIN}