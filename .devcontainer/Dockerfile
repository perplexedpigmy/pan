# [Choice] Debian OS version (use bookworm, or bullseye on local arm64/Apple Silicon): bookworm, buster, bullseye
ARG VARIANT="bookworm"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-1-${VARIANT}


ARG USERNAME=vscode
RUN \
    # Persist history between different builds
    SNIPPET="PROMPT_COMMAND='history -a' && export HISTFILE=/cmdhistory/.bash_history" \
    && mkdir /cmdhistory \
    && touch /cmdhistory/.bash_history \
    && chown -R $USERNAME /cmdhistory/.bash_history \
    && echo $SNIPPET >> /home/$USERNAME/.bashrc \
    # 
    # 
    && apt-get update && export DEBIAN_FRONTEND=noninteractive \
    #
    # 
    # Remove imagemagick due to https://security-tracker.debian.org/tracker/CVE-2019-10131
    && apt-get purge -y imagemagick imagemagick-6-common \
    #
    # Install local cargo tools
    && sudo -i -u $USERNAME echo $CARGO_HOME \
    && sudo -i -u $USERNAME rustup default stable \
    && for package in cargo-audit cargo-binstall cargo-bundle cargo-deny cargo-expand cargo-llvm-cov cargo-make cargo-mobile cargo-nextest cargo-watch cargo-web; do \
       sudo -i -u $USERNAME cargo install --root $CARGO_HOME $package; \
    done \
    #
    #  Install bin utility and then use it to install deseriable binaries
    #
    && sudo -i -u $USERNAME wget -O /tmp/bin https://github.com/marcosnils/bin/releases/download/v0.17.5/bin_0.17.5_linux_386 \
    && sudo -i -u $USERNAME chmod a+x /tmp/bin \
    && sudo -i -u $USERNAME /tmp/bin install github.com/marcosnils/bin \
    && sudo -i -u $USERNAME rm /tmp/bin \
    #
    # List of possible utlities to install using bin
    # bat    - github.com/sharkdp/bat           modern cat
    # bottom - github.com/ClementTsang/bottom   modern top
    # broot  - github.com/Canop/broot           directory navigator 
    # navi   - github.com/denisidoro/navi       commandline cheatsheet
    # direnv - github.com/direnv/direnv         ENV secrets 
    # just   - github.com/casey/just            Make file replacement with .env awareness
    # yq     - github.com/mikefarah/yq          TOML,JSON etc processor
    # duf    - github.com/muesli/duf            df replacment 
    # age    - github.com/muesli/duf            File encryption done right 
    # sops   - github.com/mozilla/sops          JSON, TOML etc value encryption
    #
    && apt-get clean  \
    && rm -rf /var/lib/apt/lists/* 
USER $USERNAME
