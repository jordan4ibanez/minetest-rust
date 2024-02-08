#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
  let mut res = winres::WindowsResource::new();
  // ? Embed Windows manifest fixes here.
  res.set_manifest(r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0" xmlns:asmv3="urn:schemas-microsoft-com:asm.v3">
    <asmv3:application>
      <asmv3:windowsSettings>
        <dpiAware xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">true</dpiAware>
      </asmv3:windowsSettings>
    </asmv3:application>
  </assembly>
  "#);
  res.compile().unwrap();
}

#[cfg(unix)]
fn main() {}
