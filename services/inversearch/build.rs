fn main() {
    let proto_file = "./proto/inversearch.proto";

    tonic_build::configure()
        .build_server(true)
        .compile(&[proto_file], &["proto/"])?;
}
