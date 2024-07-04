#!/bin/bash
set -e -x

# Function to download and install GraalVM
install_graalvm() {
  local arch=$1
  local dist=$2

  local jdk_version="22"
  local jdk_arch=""
  local download_url=""
  local graalvm_install_dir="/opt/hostedtoolcache"
  
  if [ "$arch" == "x86_64" ]; then
    jdk_arch="x64"
  elif [ "$arch" == "aarch64" ]; then
    jdk_arch="aarch64"
  else
    echo "Unsupported architecture: $arch"
    exit 1
  fi

  if [ "$dist" == "graalvm" ]; then
    download_url="https://download.oracle.com/graalvm/${jdk_version}/latest/graalvm-jdk-${jdk_version}_linux-${jdk_arch}_bin.tar.gz"
  elif [ "$dist" == "graalvm-ce" ]; then
    download_url="https://github.com/graalvm/graalvm-ce-builds/releases/download/jdk-${jdk_version}.0.1/graalvm-community-jdk-${jdk_version}.0.1_linux-${jdk_arch}_bin.tar.gz"
  else
    echo "Unsupported distribution: $dist"
    exit 1
  fi
 
  
  # Create installation directory
  rm -rf "$graalvm_install_dir" 2>/dev/null
  mkdir -p $graalvm_install_dir

  # Download and extract GraalVM
  echo "Downloading GraalVM from $download_url"
  curl -L $download_url | tar -xz -C $graalvm_install_dir

  # Get the single directory name within the base path
  local graalvm_home=$(find "$graalvm_install_dir" -mindepth 1 -maxdepth 1 -type d | head -n 1)

  echo "GraalVM $graalvm_version installed successfully in $graalvm_home"

  # persist the var across step of github workflow
  export JAVA_HOME=$graalvm_home
  export GRAALVM_HOME=$graalvm_home
  echo "JAVA_HOME=$JAVA_HOME" >> $GITHUB_ENV
  echo "GRAALVM_HOME=$GRAALVM_HOME" >> $GITHUB_ENV

}

# Check for correct number of arguments
if [ "$#" -ne 2 ]; then
  echo "Usage: $0 <x86_64|aarch64> <graalvm|graalvm-ce>"
  exit 1
fi

# Get the architecture from the input argument
target_arch=$1
target_dist=$2

# Call the installation function
install_graalvm $target_arch $target_dist
