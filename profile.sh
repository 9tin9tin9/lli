#!/bin/bash
cargo flamegraph --dev a.lli
open flamegraph.svg -a safari
