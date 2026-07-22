#[cfg(windows)]
fn main() {
    // Generate ICO from PNG if it doesn't exist
    if !std::path::Path::new("assets/app_icon.ico").exists() {
        if let Ok(img) = image::open("assets/app_icon.png") {
            let resized = img.resize_exact(256, 256, image::imageops::FilterType::Lanczos3);
            let _ = resized.save_with_format("assets/app_icon.ico", image::ImageFormat::Ico);
        }
    }

    let mut res = winres::WindowsResource::new();
    res.set_manifest(r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
<trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
        <requestedPrivileges>
            <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
        </requestedPrivileges>
    </security>
</trustInfo>
</assembly>
"#);
    res.set_icon("assets/app_icon.ico");
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {
}
