use std::fs::{self, File};
use std::path::Path;

pub async fn ensure_vendor_assets(assets_root: &Path) -> Result<(), String> {
    let vendor = assets_root.join("vendor");
    let monaco_vs = vendor.join("monaco").join("vs");
    let xterm = vendor.join("xterm");
    let marked = vendor.join("marked");
    let dompurify = vendor.join("dompurify");

    let monaco_ok = monaco_vs.join("editor").join("editor.main.js").exists()
        && monaco_vs.join("loader.min.js").exists()
        && monaco_vs.join("nls.js").exists();
    if monaco_ok && xterm.join("xterm.min.js").exists() {
        return Ok(());
    }

    fs::create_dir_all(&monaco_vs).map_err(|e| e.to_string())?;
    fs::create_dir_all(&xterm).map_err(|e| e.to_string())?;
    fs::create_dir_all(&marked).map_err(|e| e.to_string())?;
    fs::create_dir_all(&dompurify).map_err(|e| e.to_string())?;

    // monaco tgz
    let url = "https://registry.npmjs.org/monaco-editor/-/monaco-editor-0.45.0.tgz";
    let bytes = download_bytes(url).await?;
    extract_monaco_tgz(&bytes, &monaco_vs)?;

    // xterm + addons
    download_to(
        &xterm.join("xterm.css"),
        "https://unpkg.com/xterm@5.5.0/css/xterm.css",
    )
    .await
    .ok();
    download_to(
        &xterm.join("xterm.min.js"),
        "https://unpkg.com/xterm@5.5.0/lib/xterm.min.js",
    )
    .await
    .ok();
    download_to(
        &xterm.join("xterm-addon-fit.min.js"),
        "https://unpkg.com/xterm-addon-fit@0.8.0/lib/xterm-addon-fit.min.js",
    )
    .await
    .ok();
    download_to(
        &xterm.join("xterm-addon-web-links.min.js"),
        "https://unpkg.com/xterm-addon-web-links@0.8.0/lib/xterm-addon-web-links.min.js",
    )
    .await
    .ok();

    // marked + dompurify
    download_to(
        &marked.join("marked.min.js"),
        "https://unpkg.com/marked@12.0.2/marked.min.js",
    )
    .await
    .ok();
    download_to(
        &dompurify.join("purify.min.js"),
        "https://unpkg.com/dompurify@3.1.6/dist/purify.min.js",
    )
    .await
    .ok();

    Ok(())
}

async fn download_bytes(url: &str) -> Result<bytes::Bytes, String> {
    let client = reqwest::Client::builder()
        .build()
        .map_err(|e| e.to_string())?;
    let resp = client.get(url).send().await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("http {}", resp.status()));
    }
    resp.bytes().await.map_err(|e| e.to_string())
}

async fn download_to(path: &Path, url: &str) -> Result<(), String> {
    let b = download_bytes(url).await?;
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let mut f = File::create(path).map_err(|e| e.to_string())?;
    use std::io::Write;
    f.write_all(&b).map_err(|e| e.to_string())?;
    Ok(())
}

fn extract_monaco_tgz(tgz: &bytes::Bytes, dst_vs: &Path) -> Result<(), String> {
    use flate2::read::GzDecoder;
    use std::io::Cursor;
    let dec = GzDecoder::new(Cursor::new(tgz));
    let mut ar = tar::Archive::new(dec);
    for entry in ar.entries().map_err(|e| e.to_string())? {
        let mut e = entry.map_err(|e| e.to_string())?;
        let path = e.path().map_err(|e| e.to_string())?;
        // filter for package/min/vs/**
        let comps: Vec<_> = path.components().collect();
        if comps.len() >= 3
            && comps[0].as_os_str() == "package"
            && comps[1].as_os_str() == "min"
            && comps[2].as_os_str() == "vs"
        {
            let rel = path
                .strip_prefix("package/min")
                .map_err(|e| e.to_string())?;
            let out = dst_vs.parent().unwrap_or(dst_vs).join(rel);
            if let Some(parent) = out.parent() {
                std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            e.unpack(&out).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
