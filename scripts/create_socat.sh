#!/bin/bash
# socat -d -d pty,raw,echo=0,link=/dev/virtualcom0 -
# sudo pkill socat
sleep 1
socat -d -d pty,raw,echo=0,link=/tmp/ttyV0,b115200 pty,raw,echo=0,link=/tmp/ttyV1,b115200
sudo chmod 666 /tmp/ttyV1 /tmp/ttyV0 
sudo usermod -a -G dialout $(whoami)