extern crate capnp;
extern crate multiqueue;

pub mod topic_capnp {
    include!(concat!(env!("OUT_DIR"), "/topic_capnp.rs"));
}

use topic_capnp::*;

use std::sync::Arc;
use std::thread;


fn main() {
    let mut point_builder = ::capnp::message::Builder::new_default();
    {
        let mut point = point_builder.init_root::<point::Builder>();
        point.set_x(1.0);
        point.set_y(2.0);
        point.set_z(3.0);
    }

    let msg = Arc::new(::capnp::serialize::write_message_to_words(&point_builder));

    let (send, recv) = multiqueue::broadcast_queue::<Arc<Vec<::capnp::Word>>>(4);
    let mut handles = vec![];

    for i in 0..2 {
        let cur_recv = recv.add_stream();
        handles.push(thread::spawn(move || for val in cur_recv {
            let msg = ::capnp::serialize::read_message_from_words(
                &val,
                ::capnp::message::ReaderOptions::default(),
            ).unwrap();

            let point = msg.get_root::<point::Reader>().unwrap();

            println!(
                "Stream {} got {:?} ({:?})",
                i,
                point.get_y(),
                val.as_ref() as *const _
            );
        }));
    }


    recv.unsubscribe();

    for i in 0..10 {
        loop {
            if send.try_send(msg.clone()).is_ok() {
                println!("Sending {} ({:?})", i, msg.as_ref() as *const _);
                break;
            }
        }
    }
    drop(send);

    for t in handles {
        t.join().unwrap();
    }
}
