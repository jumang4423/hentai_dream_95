#!/bin/bash

rm ~/.profile
cp ./scripts/profile ~/.profile

source ~/.profile

cargo run