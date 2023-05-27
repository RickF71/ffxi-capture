use pcap::{Device, Packet};

fn main() {
    let mut cap = match Device::lookup() {
        Ok(Some(device)) => device.open().unwrap(),
        Err(e) => panic!("Could not find device: {}", e),
    };

    loop {
        let mut buf = [0; 65536];
        let n = cap.next_packet().unwrap();

        if n == 0 {
            break;
        }

        let packet = Packet::from_slice(&buf[..n]).unwrap();
        println!("Received packet: {:?}", packet);
    }
}
