fn main() {
    println!(
        "{built_on}\n{c_flags}\n{dir}\n{platform}\n{version}\n{number}",
        built_on = openssl::version::built_on(),
        c_flags = openssl::version::c_flags(),
        dir = openssl::version::dir(),
        platform = openssl::version::platform(),
        version = openssl::version::version(),
        number = openssl::version::number(),
    );
}
