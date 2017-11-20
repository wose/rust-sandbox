extern crate capnpc;

fn main() {
    ::capnpc::CompilerCommand::new()
        .src_prefix("capnp/common_msg")
        .file("capnp/common_msg/topic.capnp")
        .run()
        .expect("capnp compile error");
}
