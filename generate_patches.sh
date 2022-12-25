#!/usr/bin/sh

cd vendor/egui_dock
git diff > ../patches/egui_dock.patch
cd ../..
