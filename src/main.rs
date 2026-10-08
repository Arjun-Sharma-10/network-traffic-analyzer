fn main() {
    fn take_ownership_str(name: String) {
        println!("Took ownership of {}, name")
    }

    fn copy_value_int(num: u32) {
        println!("The copied value is {} at {}", num, &num);
    }

    let id = String::from("packet-id");
    let port_no: u32 = 67;

    println!("{}", id);
    println!("{}", port_no);
    println!("{:p}", &port_no);

    take_ownership_str(id);
    copy_value_int(port_no);

    // if this is uncommented, the compiler gives an error as the ownership is transferred when the take_ownership_str function is called
    // println!("{id}");
    println!("{}", port_no);
    println!("{:p}", &port_no);

    /*
    bit 0: endianness (0 -> little endian; 1 -> big endian)
    bit 1: whether the system is active
    bit 2: whether strict mode is enabled
    bits 3 - 34: current system ip address (IPv4)
    bits 35 - 42: transfer protocol
    bits 43 - 58: current packet ID
    bits 59 - 62: current packet version
    bit 63: whether current packet is able to be processed (system only processes IPv4)

    The below variable states:
    endianness - big endian
    system - active
    strict mode - disabled
    system ip address - all zeroes
    transfer protocol - TCP
    current packet ID - all zeroes
    current packet version - IPv4
    processable - yes
    */
    let state: u64 = 1100000000000000000000000000000000000000110000000000000000001001;
}
