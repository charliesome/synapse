#![allow(deprecated)]

extern crate amy;
extern crate byteorder;
extern crate rand;
extern crate sha1;
extern crate url;
extern crate reqwest;
#[macro_use]
extern crate lazy_static;
extern crate pbr;
extern crate net2;
extern crate serde;
extern crate serde_json;
extern crate tiny_http;
#[macro_use]
extern crate serde_derive;

mod bencode;
mod torrent;
mod util;
mod socket;
mod disk;
mod tracker;
mod control;
mod listener;
mod rpc;
mod throttle;

use std::{env, io, thread, time};
use std::sync::atomic;
use std::fs::File;

lazy_static! {
    pub static ref PEER_ID: [u8; 20] = {
        use rand::{self, Rng};

        let mut pid = [0u8; 20];
        let prefix = b"-SN0001-";
        for i in 0..prefix.len() {
            pid[i] = prefix[i];
        }

        let mut rng = rand::thread_rng();
        for i in 8..19 {
            pid[i] = rng.gen::<u8>();
        }
        pid
    };

    pub static ref PORT: atomic::AtomicUsize = {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(5000, 30000);
        println!("Listening on port {:?}", num);
        atomic::AtomicUsize::new(num)
    };

    pub static ref DISK: disk::Handle = {
        disk::start()
    };

    pub static ref CONTROL: control::Handle = {
        control::start()
    };

    pub static ref TRACKER: tracker::Handle = {
        tracker::start()
    };

    pub static ref LISTENER: listener::Handle = {
        listener::start()
    };

    pub static ref RPC: rpc::Handle = {
        rpc::start()
    };
}

fn main() {
    // lol
    LISTENER.init();
    RPC.init();
    let torrent = env::args().nth(1).unwrap();
    download_torrent(&torrent).unwrap();
    thread::sleep(time::Duration::from_secs(99999));
    println!("Done");
}

fn download_torrent(path: &str) -> Result<(), io::Error> {
    let mut data = File::open(path)?;
    let b = bencode::decode(&mut data).unwrap();
    CONTROL.ctrl_tx.lock().unwrap().send(control::Request::AddTorrent(b)).unwrap();
    Ok(())
}
