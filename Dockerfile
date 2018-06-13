FROM amazonlinux:latest

# Copy code from executing directory
WORKDIR /work
ADD . /work

# Install devtools
RUN yum install -y shadow-utils.x86_64 gcc gcc-c++ clang libgcc cmake git \
  && yum clean all

# Create snips-nlu-js user to avoid 'cannot run in wd %s %s error'
RUN groupadd --gid 1000 snips-nlu-js \
  && useradd --uid 1000 --gid snips-nlu-js --shell /bin/bash --create-home snips-nlu-js

# Give snips-nlu-js user permission to use workdir
RUN chown -R snips-nlu-js:snips-nlu-js /work

# Switch to snips-nlu-js user
USER snips-nlu-js

# Install rust stable
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH $PATH:/home/snips-nlu-js/.cargo/bin

# Install nvm
RUN curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.33.11/install.sh | bash
ENV NVM_DIR=/home/snips-nlu-js/.nvm

# Install lambda node versions (8.10.0, 6.10.0)
RUN . $HOME/.nvm/nvm.sh \ 
    && nvm install 6.10.0 \
    && nvm install 8.10.0 \
    && nvm alias default 8.10.0 \
    && nvm use default

# add node and npm to path so the commands are available
ENV NODE_PATH $NVM_DIR/v8.10.0/lib/node_modules
ENV PATH $NVM_DIR/versions/node/v8.10.0/bin:$PATH

RUN node -v

CMD ["/bin/bash"]
