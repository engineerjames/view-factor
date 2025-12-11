set -e
echo "Cleaning up previous build if it exists..."
rm -rf ./dist ./target
cargo build --release

echo "Copying binary files to dist..."
mkdir ./dist/
cp ./target/release/view-factor ./dist/