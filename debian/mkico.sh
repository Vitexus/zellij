#!/bin/bash
PACKAGE=zellij
SRC="${PACKAGE}.svg"

resolutions='16 32 48 64 128 256 512 1024'

for resolution in $resolutions
do
    echo $resolution
    mkdir -p debian/zellij/usr/share/icons/${resolution}x${resolution}/
    inkscape -z -w ${resolution} -h ${resolution} $SRC -o debian/zellij/usr/share/icons/${resolution}x${resolution}/${PACKAGE}.png
done
echo All done
