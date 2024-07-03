#!/bin/bash
set -e -x

yum install -y zip

curl -s "https://get.sdkman.io" | sh -s -- -y
source "$HOME/.sdkman/bin/sdkman-init.sh"

sdk install java 22.0.1-graalce
sdk default java 22.0.1-graalce