fn main() {
    let (chan, port) = channel();

    spawn(proc() {
        chan.send(10u);
    });

    println!("{:s}", port.recv().to_str());
}
