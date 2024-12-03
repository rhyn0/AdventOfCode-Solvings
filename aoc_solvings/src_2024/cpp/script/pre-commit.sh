#!/usr/bin/env sh

if [ ! -f "./build/compile_commands.json" ]; then
    echo "Compiler commands file does not exist, clang-tidy can't be run."
    echo "Exiting ..."
    exit 1
fi

clang-format --style=file --Werror -i src/**/*.{c,h}pp
clang-tidy -p build src/**/*.{c,h}pp
