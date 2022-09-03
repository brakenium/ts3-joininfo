#!/bin/sh
target="x86_64-unknown-linux-gnu"
ts3_package_dir="./ts3_package"
ts3_plugin_dll_dir="${ts3_package_dir}/plugins"
plugin_name="ts3joininfo"
ts3_plugin_output="./${plugin_name}.ts3_plugin"
release_output="./target/${target}/release"
cargo_output_dll="$release_output/lib${plugin_name}.so"
plugin_dll_prefix="${ts3_plugin_dll_dir}/${plugin_name}_linux_"
plugin_dll_x86="${plugin_dll_prefix}x86.so"
plugin_dll_x64="${plugin_dll_prefix}amd64.so"

plugin_package_output="ts3joininfo.ts3_plugin"

echo $plugin_dll_path
cargo build --release --target $target
rm -rf $ts3_plugin_dll_dir/*
cp $cargo_output_dll $plugin_dll_x86
cp $cargo_output_dll $plugin_dll_x64
rm $plugin_package_output || true
cd "./ts3_package/"
zip -r "../$plugin_package_output" .