#!/bin/bash

systemName="$(uname -s)"
if [[ $systemName = "Linux" || $systemName = "Darwin" || $systemName = "CYGWIN" || $systemName = "MINGW" ]]
then
    { cargo build --release && sudo cp ./target/release/cpphelp /usr/local/bin/ && echo "cpphelp successfully installed."; } || \
    echo "Failed to install cpphelp."
else
    mkdir ~/bin
    { cargo build --release && sudo cp ./target/release/cpphelp ~/bin/ && echo "cpphelp successfully installed."; } || \
    echo "Failed to install cpphelp."
fi