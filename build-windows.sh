set -e

cd $(git rev-parse --show-toplevel)
cargo build --target=x86_64-pc-windows-gnu --release
trash-put output
mkdir output
cp target/x86_64-pc-windows-gnu/release/DuckSlayer.exe ./output
cp -r assets ./output
zip output/DuckSlayer.zip output/*
echo DONE, see DuckSlayer.zip
