fn main() {
    fn take_ownership_str(name: String) {
        println!("Took ownership of {}", name)
    }

    fn copy_value_int(num: u32) {
        println!("The copied value is {} at {}\n", num, &num);
    }

    let id = String::from("packet-id");
    let port_no: u32 = 67;

    println!("{}", id);
    println!("{}", port_no);
    println!("{:p}\n", &port_no);

    take_ownership_str(id);
    copy_value_int(port_no);

    // if this is uncommented, the compiler gives an error as the ownership is transferred when the take_ownership_str function is called
    // println!("{id}");
    println!("{}", port_no);
    println!("{:p}\n\n", &port_no);

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
    let state_be: u64 = 0b1100000000000000000000000000000000000000110000000000000000001001;
    let state_le: u64 = 0b0100000000000000000000000000000000001100000000000000000000000101;

    // extracting bits from a value
    fn extract_bits(val: u64, start: u8, end: u8) -> u64 {
        let mut mask: u64 = 0b0;

        for _n in 0..(end - start) {
            mask = mask * 2 + 1;
        }
        for _n in 0..(64 - end) {
            mask *= 2;
        }

        return (val & mask) >> (64 - end);
    }

    // adjust multi bit flags based on endianness
    fn endianness_adjusted_bits(val: u64, start: u8, end: u8) -> u64 {
        let endianness: u64 = get_endianness(val);
        let bits: u64 = extract_bits(val, start, end);
        if endianness == 0 {
            let le_bytes = bits.to_le_bytes();
            let mut be_bytes: [u8; 8] = [0; 8];
            for n in 0..=7 {
                be_bytes[7 - n] = le_bytes[n];
            }
            return u64::from_be_bytes(be_bytes);
        }
        return bits;
    }

    // gets
    fn get_endianness(val: u64) -> u64 { return extract_bits(val, 0, 1); }
    
    fn get_system_state(val: u64) -> u64 { return extract_bits(val, 1, 2); }
    
    fn get_is_strict(val: u64) -> u64 { return extract_bits(val, 2, 3); }
    
    fn get_system_ip(val: u64) -> u64 { return endianness_adjusted_bits(val, 3, 35); }
    
    fn get_transfer_protocol(val: u64) -> u64 { return endianness_adjusted_bits(val, 35, 43); }
    
    fn get_packet_id(val: u64) -> u64 { return endianness_adjusted_bits(val, 43, 59); }
    
    fn get_packet_version(val: u64) -> u64 { return endianness_adjusted_bits(val, 59, 63); }
    
    fn get_processable(val: u64) -> u64 { return extract_bits(val, 63, 64); }

    let num: u32 = 0b0010;

    // testing be
    println!("{}", get_endianness(state_be));
    println!("{}", get_system_state(state_be));
    println!("{}", get_is_strict(state_be));
    println!("{}", get_system_ip(state_be));
    println!("{}", get_transfer_protocol(state_be));
    println!("{}", get_packet_id(state_be));
    println!("{}", get_packet_version(state_be));
    println!("{}\n\n", get_processable(state_be));
    
    // testing le
    println!("{}", get_endianness(state_le));
    println!("{}", get_system_state(state_le));
    println!("{}", get_is_strict(state_le));
    println!("{}", get_system_ip(state_le));
    println!("{}", get_transfer_protocol(state_le));
    println!("{}", get_packet_id(state_le));
    println!("{}", get_packet_version(state_le));
    println!("{}", get_processable(state_le));
}
