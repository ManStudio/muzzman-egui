#!/usr/bin/sh

cd vendor/egui_dock
echo "Is ok if fails patches!"
! git apply ../patches/egui_dock.patch
cd ../..
