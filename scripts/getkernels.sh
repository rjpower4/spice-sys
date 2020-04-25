#!/usr/bin/sh


if [ ! -d "thirdparty/testkernels" ]; then
    mkdir -p thirdparty/testkernels
fi

if [ ! -f "thirdparty/testkernels/de430.bsp" ]; then
    wget https://naif.jpl.nasa.gov/pub/naif/generic_kernels/spk/planets/de430.bsp
    mv de430.bsp thirdparty/testkernels/
fi

if [ ! -f "thirdparty/testkernels/naif0012.tls" ]; then
    wget https://naif.jpl.nasa.gov/pub/naif/generic_kernels/lsk/naif0012.tls
    mv naif0012.tls thirdparty/testkernels/
fi

if [ ! -f "thirdparty/testkernels/NSY_SCLKSCET.00005.tsc" ]; then
    wget https://naif.jpl.nasa.gov/pub/naif/INSIGHT/kernels/sclk/NSY_SCLKSCET.00005.tsc
    mv NSY_SCLKSCET.00005.tsc thirdparty/testkernels/NSY_SCLKSCET.00005.tsc
fi


