# Messenger Desktop (Rust + Tauri)

App desktop nhẹ cho **macOS** và **Windows**, viết bằng Rust (Tauri 2). App mở
`facebook.com/messages` — chính là Messenger bản web chính thức của Meta — nên
sticker đã lưu trong tài khoản, hiệu ứng tin nhắn, ảnh/video, thả cảm xúc...
đều là của Messenger thật, không phải bản "nhái".

> Meta đã ngừng app Messenger cho Mac/Windows (12/2025) và đóng messenger.com
> (4/2026). Hiện chỉ còn `facebook.com/messages` cho máy tính, nên app này bọc đúng
> địa chỉ đó.

## Build bằng GitHub Actions (không cần cài gì trên máy)

1. Tạo repo mới trên GitHub, upload toàn bộ thư mục này (giữ nguyên `.github/`).
2. Vào tab **Actions** → **Build Messenger Desktop** → **Run workflow**.
   (Hoặc push một tag: `git tag v0.1.0 && git push --tags`)
3. Đợi ~10–15 phút. File cài đặt nằm trong **Releases** (bản nháp / draft) của repo:
   - `.dmg` — macOS (chạy cả chip Apple lẫn Intel)
   - `.exe` / `.msi` — Windows
4. Nếu Actions báo thiếu quyền ghi: Settings → Actions → General →
   Workflow permissions → **Read and write permissions**.

## Mở app lần đầu

- **Mac**: app chưa ký bằng Apple Developer ID nên bị chặn. Chuột phải vào app →
  **Open** → **Open**. Nếu vẫn báo "damaged": `xattr -cr "/Applications/Messenger Desktop.app"`
- **Windows**: SmartScreen hiện "Windows protected your PC" → **More info** → **Run anyway**.

## Phím tắt

- `Alt + ←` (Windows) / `⌘ + [` (Mac): quay lại trang trước.

## Tùy chỉnh

- Tên app, bundle id: `src-tauri/tauri.conf.json` (`productName`, `identifier`).
- Icon: thay `app-icon.png`.
- Domain nào ở lại trong app, domain nào mở ra trình duyệt: `INTERNAL_DOMAINS` trong
  `src-tauri/src/main.rs`.

## Chạy thử trên máy (tùy chọn)

Cần Rust + Node.js:

```
npm install
npx tauri icon app-icon.png
npm run tauri dev
```

## Giới hạn cần biết

- Tính năng **bằng đúng** những gì `facebook.com/messages` hỗ trợ. Nếu bản web
  thiếu một tính năng nào đó của bản mobile thì app này cũng thiếu.
- Thông báo (notification): Windows dùng được; trên macOS, WebView hệ thống có thể
  không hiện thông báo của web.
- Gọi thoại/video phụ thuộc vào WebView của hệ điều hành; hãy thử trước khi dựa vào nó.
