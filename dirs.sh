#!/bin/bash

CONFIG=${HOME}/.config

# arr of dirs
DIRS=(
    ${CONFIG}/waybar
    ${CONFIG}/nvim
    ${CONFIG}/hypr
    ${CONFIG}/kitty
)


copy() {
    echo "Copying files..."
    for dir in ${DIRS[@]}; do
        cp -r ${dir} .
    done
}

apply() {
    echo "Applying files..."
}

case $1 in
    copy)
        copy
        ;;
    apply)
        apply
        ;;
    *)
        echo "Invalid argument"
        ;;
esac

