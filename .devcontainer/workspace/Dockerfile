FROM debian:bullseye-slim

ARG USERNAME=nonroot
ARG USER_UID=1000
ARG USER_GID=1000

WORKDIR /
ENV DEBIAN_FRONTEND=noninteractive

RUN : \
&& groupadd --gid "${USER_GID}" "${USERNAME}" \
&& useradd --create-home --uid "${USER_UID}" --gid "${USER_GID}" --shell /bin/bash "${USERNAME}" \
&& mkdir -p /etc/sudoers.d/ \
&& echo "$USERNAME" ALL=\(root\) NOPASSWD:ALL > "/etc/sudoers.d/$USERNAME" \
&& chmod 0440 "/etc/sudoers.d/$USERNAME"

USER root
RUN apt-get update && apt-get -y --no-install-recommends install \
apt-transport-https=2.2.4 \
build-essential=12.9 \
ca-certificates=20210119 \
curl=7.74.0-1.3+deb11u1 \
git=1:2.30.2-1 \
gnupg=2.2.27-2+deb11u1 \
iproute2=5.10.0-4 \
less=551-2 \
locales=2.31-13+deb11u3 \
lsb-release=11.1.0 \
lsof=4.93.2+dfsg-1.1 \
procps=2:3.3.17-5 \
python-is-python3=3.9.2-1 \
python3=3.9.2-3 \
sudo=1.9.5p2-3 \
unzip=6.0-26 \
wget=1.21-1+deb11u1 \
zip=3.0-12 \
&& apt-get -y clean \
&& rm -rf /var/lib/apt/lists/*

COPY setup.sh /tmp
COPY scripts  /tmp/scripts
RUN ./tmp/setup.sh && rm /tmp/setup.sh && rm -r /tmp/scripts
