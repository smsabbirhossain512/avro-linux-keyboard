fn main() {
    println!("cargo:rustc-link-lib=ibus-1.0");
    println!("cargo:rustc-link-lib=gobject-2.0");
    println!("cargo:rustc-link-lib=glib-2.0");
}
