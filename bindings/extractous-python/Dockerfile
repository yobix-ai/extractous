# This docker image is based on manylinux_2_28_x86_64 and installs Graalvm and Rust
# Intended to be used to build python wheels on linux on local dev machine
# ./github/workflow/release_python.yml uses maturin action with manylinux_2_28_x86_64 image
#

# Use manylinux base image
FROM quay.io/pypa/manylinux_2_28_x86_64

# Install dependencies
RUN yum update -y && \
    yum install -y zip gcc gcc-c++ make git openssl-devel bzip2-devel libffi-devel \
    wget curl unzip

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup default 1.78.0

#ENV PATH="/root/.cargo/bin:${PATH}"

# Verify Rust installation
RUN rustc --version && cargo --version

# Install GraalVM
ENV SDKMAN_DIR="/root/.sdkman"
RUN curl -s "https://get.sdkman.io" | sh -s -- -y
RUN source "$SDKMAN_DIR/bin/sdkman-init.sh" && sdk install java 22.0.1-graalce
RUN source "$SDKMAN_DIR/bin/sdkman-init.sh" && sdk use java 22.0.1-graalce

ENV JAVA_HOME="$SDKMAN_DIR/candidates/java/22.0.1-graalce"
ENV GRAALVM_HOME="$JAVA_HOME"
# Add GraalVM to the PATH
ENV PATH="${JAVA_HOME}/bin:${PATH}"

RUN ls "$SDKMAN_DIR/candidates"

RUN java --version

RUN native-image --version

# Clean up
RUN yum clean all

# Set working directory
WORKDIR /workspace

# Set default command
CMD ["/bin/bash"]
