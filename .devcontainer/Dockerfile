# [Choice] Debian OS version (use bookworm, or bullseye on local arm64/Apple Silicon): bookworm, buster, bullseye
ARG VARIANT="bookworm"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-1-${VARIANT}


ARG USERNAME=vscode
RUN \
    # Persist history between different builds
    SNIPPET="PROMPT_COMMAND='history -a' && export HISTFILE=/cmdhistory/.bash_history" \
    && mkdir /cmdhistory \
    && touch /cmdhistory/.bash_history \
    && whoami \
    && echo ... ${USERNAME} ...\
    && chown -R $USERNAME /cmdhistory/.bash_history \
    && echo $SNIPPET >> /home/$USERNAME/.bashrc \
    # 
    # 
    && apt-get update && export DEBIAN_FRONTEND=noninteractive \
    #
    # Fix rust root owned rust packages
    && chown vscode -R /usr/local/cargo/  \
    # 
    # Remove imagemagick due to https://security-tracker.debian.org/tracker/CVE-2019-10131
    && apt-get purge -y imagemagick imagemagick-6-common \
    && apt-get clean  \
    && rm -rf /var/lib/apt/lists/* 


# WORKDIR /home/$USERNAME
# COPY src src
# COPY tests tests
# COPY Cargo.toml ./

USER $USERNAME
