set -e

rm -rf ./NeoMagicPhysX/runtimes/*

# Compile
mkdir -p "NeoMagicPhysX/runtimes/linux-x64/native/"
(cd "libneomagicphysx" && cargo build --release --target "x86_64-unknown-linux-gnu")
cp "libneomagicphysx/target/x86_64-unknown-linux-gnu/release/libneomagicphysx.so" "NeoMagicPhysX/runtimes/linux-x64/native"
