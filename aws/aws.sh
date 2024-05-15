#!/bin/bash

sudo yum update -y
yum install -y docker # Amazon Linux 2023
# sudo amazon-linux-extras install docker # Amazon Linux 2
sudo service docker start
sudo docker pull shardlabs/starknet-devnet-rs
sudo docker run -d -p "80:5050" shardlabs/starknet-devnet-rs:latest