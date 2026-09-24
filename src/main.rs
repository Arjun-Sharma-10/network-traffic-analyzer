fn main() {
    fn take_ownership_str(name: String) {
        println!("Took ownership of {name}")
    }

    fn copy_value_int(num: i32) {
        println!("The copied value is {num}");
    }

    let id = String::from("packet-id");
    let port_no: i32 = 67;

    println!("{id}");
    println!("{port_no}");

    take_ownership_str(id);
    copy_value_int(port_no);

    // if this is uncommented, the compiler gives an error as the ownership is transferred when the take_ownership_str function is called
    // println!("{id}");
    println!("{port_no}");
}
