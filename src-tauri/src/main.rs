// Ẩn cửa sổ console trên Windows ở bản release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Manager, Url, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_opener::OpenerExt;

/// Messenger trên web hiện nằm ở đây (messenger.com đã bị Meta đóng từ 4/2026).
const START_URL: &str = "https://www.facebook.com/messages/";

/// Các domain được phép điều hướng ngay trong app (đăng nhập, ảnh, sticker, CDN...).
/// Mọi link khác sẽ được mở bằng trình duyệt mặc định của máy.
const INTERNAL_DOMAINS: &[&str] = &[
    "facebook.com",
    "messenger.com",
    "fbcdn.net",
    "fbsbx.com",
    "facebook.net",
    "fb.com",
    "meta.com",
    "instagram.com",
    "cdninstagram.com",
];

/// WKWebView không gửi User-Agent giống Safari thật nên Facebook có thể
/// hiện trang "trình duyệt không hỗ trợ". Giả lập Safari trên macOS.
#[cfg(target_os = "macos")]
const MAC_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) \
AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.0 Safari/605.1.15";

/// Chạy trước mỗi trang:
/// - Link `target="_blank"` được mở trong cùng cửa sổ để bộ lọc điều hướng bên dưới
///   quyết định (link ngoài -> trình duyệt, link Facebook -> ở lại app).
/// - Alt+← (Windows) hoặc ⌘+[ (Mac) để quay lại trang trước.
const INIT_SCRIPT: &str = r#"
(function () {
  if (window.top !== window) return;
  document.addEventListener('click', function (e) {
    var t = e.target;
    if (!(t instanceof Element)) return;
    var a = t.closest('a[target="_blank"]');
    if (a && a.href) {
      e.preventDefault();
      window.location.assign(a.href);
    }
  }, true);
  window.addEventListener('keydown', function (e) {
    if ((e.altKey && e.key === 'ArrowLeft') || (e.metaKey && e.key === '[')) {
      history.back();
    }
  });
})();
"#;

fn host_is_internal(host: &str) -> bool {
    INTERNAL_DOMAINS
        .iter()
        .any(|d| host == *d || host.ends_with(&format!(".{d}")))
}

fn is_internal(url: &Url) -> bool {
    match url.scheme() {
        "about" | "blob" | "data" => true,
        "http" | "https" => url.host_str().map(host_is_internal).unwrap_or(false),
        _ => false,
    }
}

/// Chỉ mở các scheme an toàn ra ngoài app.
fn open_external(app: &AppHandle, target: &str) {
    let allowed = ["http://", "https://", "mailto:", "tel:"];
    if allowed.iter().any(|p| target.starts_with(p)) {
        let _ = app.opener().open_url(target.to_string(), None::<String>);
    }
}

fn main() {
    tauri::Builder::default()
        // Single-instance phải đăng ký đầu tiên.
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.unminimize();
                let _ = w.show();
                let _ = w.set_focus();
            }
        }))
        .plugin(tauri_plugin_opener::init())
        // Nhớ kích thước / vị trí cửa sổ giữa các lần mở.
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app| {
            let handle = app.handle().clone();

            let builder = WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External(START_URL.parse::<Url>().expect("invalid START_URL")),
            )
            .title("Messenger")
            .inner_size(1200.0, 800.0)
            .min_inner_size(480.0, 600.0)
            .initialization_script(INIT_SCRIPT)
            .on_navigation(move |url| {
                // l.facebook.com/l.php?u=<link thật> -> mở thẳng link thật ra trình duyệt.
                if url.host_str() == Some("l.facebook.com") && url.path().starts_with("/l.php") {
                    if let Some((_, real)) = url.query_pairs().find(|(k, _)| k == "u") {
                        open_external(&handle, &real);
                        return false;
                    }
                }
                if is_internal(url) {
                    return true;
                }
                open_external(&handle, url.as_str());
                false
            });

            #[cfg(target_os = "macos")]
            let builder = builder.user_agent(MAC_USER_AGENT);

            builder.build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Messenger Desktop");
}
