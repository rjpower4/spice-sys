#!/usr/bin/sh

tarfile="cspice.tar.Z"

if [ ! -d "$OUT_DIR/thirdparty" ]; then
    mkdir $OUT_DIR/thirdparty
fi

if [ ! -d "$OUT_DIR/thirdparty/cspice" ]; then
    if [ ! -f "$OUT_DIR/thirdparty/${tarfile}" ]; then
        wget "http://naif.jpl.nasa.gov/pub/naif/toolkit//C/PC_Linux_GCC_64bit/packages/cspice.tar.Z" -P $OUT_DIR/thirdparty/
    fi
    tar -xf $OUT_DIR/thirdparty/$tarfile -C $OUT_DIR/thirdparty/

    mv $OUT_DIR/thirdparty/cspice/lib/cspice.a $OUT_DIR/thirdparty/cspice/lib/libcspice.a
    mv $OUT_DIR/thirdparty/cspice/lib/csupport.a $OUT_DIR/thirdparty/cspice/lib/libcsupport.a
fi
