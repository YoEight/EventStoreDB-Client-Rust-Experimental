pub fn generate() {
    let out_dir = "src/event_store/client";
    let files = [
        "protos/persistent.proto",
        "protos/streams.proto",
        "protos/shared.proto",
        "protos/gossip.proto",
    ];

    tonic_build::configure()
        .build_server(false)
        .out_dir(out_dir)
        .compile(&files, &["protos"])
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));

    let gen_dir = std::fs::read_dir(out_dir).unwrap();

    for entry in gen_dir {
        let file = entry.unwrap();
        let filename_string = file.file_name().into_string().unwrap();
        if filename_string.starts_with("event_store.client.") {
            let remaining = filename_string.trim_start_matches("event_store.client.");
            let new_file_name = if remaining == "persistent_subscriptions.rs" {
                "persistent.rs"
            } else {
                remaining
            };

            let new_file = file.path().parent().unwrap().join(new_file_name);

            std::fs::rename(file.path(), new_file).unwrap();
        }
    }
}

fn main() {
    generate();
}
