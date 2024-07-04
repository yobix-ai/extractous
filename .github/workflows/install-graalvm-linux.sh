#!/bin/bash
set -e -x



uname -a
apt install -y zip

curl -s "https://get.sdkman.io" | sh -s -- -y
source "$HOME/.sdkman/bin/sdkman-init.sh"

sdk install java 22.0.1-graalce
sdk default java 22.0.1-graalce

echo "GRAALVM_HOME: $GRAALVM_HOME"
echo "JAVA_HOME: $JAVA_HOME"
java --version
native-image --version