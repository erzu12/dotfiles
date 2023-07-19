#!/bin/bash

CONFIG=${HOME}/.config
EWW=${CONFIG}/eww
NVIM=${CONFIG}/nvim
HYPR=${CONFIG}/hypr

# arr of dirs
DIRS=(
    ${CONFIG}/eww
    ${CONFIG}/nvim
    ${CONFIG}/hypr
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

