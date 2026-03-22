#!/bin/bash
# Setup network bridge for PiNetCore inline mode

INTERFACE_WAN=$1
INTERFACE_LAN=$2

sudo ip link add name br0 type bridge
sudo ip link set $INTERFACE_WAN master br0
sudo ip link set $INTERFACE_LAN master br0
sudo ip link set br0 up
sudo ip addr flush dev $INTERFACE_LAN
sudo ip addr add 192.168.1.1/24 dev br0

echo "Bridge br0 ready: WAN=$INTERFACE_WAN LAN=$INTERFACE_LAN"

