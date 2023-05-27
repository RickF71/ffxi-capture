use pcap::{PcapHandle, PcapFilter};

fn main() {
  // Create a new instance of the PcapHandle struct.
  let mut pcap_handle = PcapHandle::new("eth0").unwrap();

  // Set the filter for the PcapHandle struct.
  let filter = PcapFilter::new("udp and dst port 25565").unwrap();
  pcap_handle.set_filter(filter).unwrap();

  // Start capturing data.
  pcap_handle.capture().unwrap();

  // Get the data that was captured.
  let data = pcap_handle.data().unwrap();

  // Do something with the data.
  println!("{:?}", data);
}