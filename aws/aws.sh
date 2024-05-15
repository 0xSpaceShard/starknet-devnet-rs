#!/bin/bash

# TODO: test and use -> sudo usermod -a -G docker ec2-user

sudo yum update -y

# Amazon Linux 2023
yum install -y docker

# Amazon Linux 2
# sudo amazon-linux-extras install docker

sudo service docker start
sudo docker pull shardlabs/starknet-devnet-rs
sudo docker run -d -p "80:5050" shardlabs/starknet-devnet-rs:latest