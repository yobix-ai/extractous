#!/bin/bash
set -e -x

# Check for correct number of arguments
if [ "$#" -ne 1 ]; then
  echo "Usage: $0 <graalvm-sdkman-version>"
  echo " Installs graalvm using sdkman, example: $0 23.0.1-graalce"
  exit 1
fi

# Get the version from the input argument
jdk_version=$1

curl -s "https://get.sdkman.io" | sh -s -- -y
source "$HOME/.sdkman/bin/sdkman-init.sh"


sdk install java $jdk_version
sdk default java $jdk_version

# Verify
echo "GRAALVM_HOME: $GRAALVM_HOME"
echo "JAVA_HOME: $JAVA_HOME"
java --version
native-image --version