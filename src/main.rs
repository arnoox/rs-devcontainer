use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:34254")?;
        let addr = socket.local_addr()?;

        let send_buf: Vec<u8> = (0..100).collect();
        socket.send_to(&send_buf, addr)?;

        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 200];
        let (amt, src) = socket.recv_from(&mut buf)?;

        println!("received {} bytes from {}: {:?}", amt, src, &buf[..amt]);
    } // the socket is closed here
    Ok(())
}
