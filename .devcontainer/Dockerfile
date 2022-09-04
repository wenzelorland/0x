# Define base image
FROM ubuntu:22.04 

ARG USER

ENV PATH="/root/miniconda3/bin:${PATH}"
ARG PATH="/root/miniconda3/bin:${PATH}"

RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install -y \
    ca-certificates \
    curl \
    git \
    libssl-dev \
    wget \
    cmake \
    && rm -rf /var/lib/apt/lists/*

# Use for Arm64 infrastructure
ARG MINICONDA3=Miniconda3-latest-Linux-aarch64.sh
ARG PYTHON_VERSION=3.9
# Use for x86_64 infrastructure
#ENV MINICONDA3=Miniconda3-latest-Linux-x86_64.sh

RUN wget https://repo.anaconda.com/miniconda/$MINICONDA3 \
    && mkdir /root/.conda \
    && bash $MINICONDA3 -b \
    && rm -f $MINICONDA3 

ADD dependencies.txt /tmp/dependencies.txt
RUN conda create -n 0x python=$PYTHON_VERSION --file /tmp/dependencies.txt \ 
    && rm /tmp/dependencies.txt \
    && conda init bash \
    && echo 'conda activate 0x' >> ~/.bashrc 

# Install Rust
#RUN curl https://sh.rustup.rs -sSf | sh -s -- -y 
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add .cargo/bin to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Install NVM and Node
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | sh
ENV NVM_DIR=/root/.nvm
ENV NODE_VERSION=16.15.1

# install node and npm
RUN . $NVM_DIR/nvm.sh \
    && nvm install $NODE_VERSION \
    && nvm alias default $NODE_VERSION \
    && nvm use default

# add node and npm to path so the commands are available
ENV NODE_PATH $NVM_DIR/v$NODE_VERSION/lib/node_modules
ENV PATH $NVM_DIR/versions/node/v$NODE_VERSION/bin:$PATH

RUN npm -v
RUN node -v
#RUN . ~/.bashrc && nvm --version 

# Check cargo is visible
RUN cargo --version
RUN conda --version