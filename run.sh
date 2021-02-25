cargo build-bpf --manifest-path=./Cargo.toml --bpf-out-dir=dist/program
solana program deploy /home/ananas/Solana/Solana-Mixer/dist/program/solana_mixer.so
