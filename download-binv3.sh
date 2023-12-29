#!/bin/bash

base_url="https://zmann-bucket.onrender.com/artifacts/Toybox_c1200"
files=("Accordion.binv3" "AltoSax.binv3" "Bandoneon.binv3" "Brass1.binv3" "Brass2.binv3" "BrassEnsemble.binv3" "Cello.binv3" "ChurchOrgan.binv3" "Clarinet.binv3" "ElecOrgan1.binv3" "ElecOrgan2.binv3" "ElecOrgan3.binv3" "ElecOrgan4.binv3" "Flute.binv3" "FrenchHorn1.binv3" "FrenchHorn2.binv3" "Harmonica.binv3" "Harp.binv3" "Oboe.binv3" "Piccolo.binv3" "PipeOrgan.binv3" "Recorder.binv3" "ReedOrgan.binv3" "SopranoSax.binv3" "Soundtrack.binv3" "Strings1.binv3" "Strings2.binv3" "Strings3.binv3" "SynPad1.binv3" "SynPad2.binv3" "SynPad3.binv3" "TenorSax.binv3" "Trumpet.binv3" "Tuba.binv3" "Violin.binv3")

output_dir="./samples/Toybox_c1200"

# Create the output directory if it doesn't exist
mkdir -p "$output_dir"

for file in "${files[@]}"
do
  wget -P "$output_dir" "$base_url/$file"
done