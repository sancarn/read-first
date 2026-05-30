fn main() {
    println!("cargo:rerun-if-changed=assets/app.ico");

    #[cfg(windows)]
    {
        winresource::WindowsResource::new()
            .set_icon("assets/app.ico")
            .compile()
            .expect("embed app icon");
    }
}
