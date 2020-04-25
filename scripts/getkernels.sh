#!/usr/bin/sh

if [ ! -d "thirdparty/testkernels" ]; then
    mkdir -p thirdparty/testkernels
    wget https://naif.jpl.nasa.gov/pub/naif/generic_kernels/spk/planets/de430.bsp
    wget https://naif.jpl.nasa.gov/pub/naif/generic_kernels/lsk/naif0012.tls
    mv de430.bsp thirdparty/testkernels/
    mv naif0012.tls thirdparty/testkernels/
fi

