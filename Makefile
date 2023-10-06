default: debug

debug:
   cargo build
   mv target/debug/shard ./

release:
   cargo build --release
   mv target/release/shard ./

install: release
   sudo cp shard /usr/local/bin/shard

clean:
   cargo clean
