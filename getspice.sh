#!/usr/bin/sh

tarfile="cspice.tar.Z"

if [ ! -d "thirdparty" ]; then
    mkdir thirdparty
fi

if [ ! -d "thirdparty/cspice" ]; then
    if [ ! -f "thirdparty/${tarfile}" ]; then
        wget "http://naif.jpl.nasa.gov/pub/naif/toolkit//C/PC_Linux_GCC_64bit/packages/cspice.tar.Z"
        mv $tarfile thirdparty/$tarfile
    fi
    tar -xf thirdparty/$tarfile -C thirdparty/

    mv thirdparty/cspice/lib/cspice.a thirdparty/cspice/lib/libcspice.a
    mv thirdparty/cspice/lib/csupport.a thirdparty/cspice/lib/libcsupport.a
fi
