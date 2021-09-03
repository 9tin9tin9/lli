#!/bin/bash
cargo build
sudo cargo flamegraph --dev a.lli
open flamegraph.svg -a safari
