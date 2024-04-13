#!/bin/bash
sudo chmod 666 /tmp/ttyV1 /tmp/ttyV0 
sudo usermod -a -G dialout $(whoami)