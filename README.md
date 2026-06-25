# 我的电视 - 桌面版

基于 Tauri + Vue 3 的 IPTV 桌面播放器，支持 Windows / Linux。

**用户无需安装任何额外软件**，下载安装包直接使用。

## 下载

从 [Releases](../../releases) 页面下载对应平台的安装包：

| 平台 | 文件 |
|------|------|
| Windows | `mytv-desktop_x.x.x_x64-setup.exe` |
| Linux (deb) | `mytv-desktop_x.x.x_amd64.deb` |
| Linux (AppImage) | `mytv-desktop_x.x.x_amd64-bundled.AppImage` |

## 使用

1. 首次启动点击右上角 **"测速更新"**，等待 2～5 分钟
2. 测速完成后频道列表自动加载
3. 点击频道即可播放

## 频道数据位置

| 平台 | 路径 |
|------|------|
| Windows | `%APPDATA%\mytv-desktop\iptv_sources.m3u8` |
| Linux | `~/.config/mytv-desktop/iptv_sources.m3u8` |

## 本地开发

```bash
# 需要安装
# - Rust https://rustup.rs
# - Node.js >= 18
# - libmpv-dev (Linux: sudo apt install libmpv-dev)
# - Windows: 见 .github/workflows/build.yml 中的 mpv-dev 下载步骤

npm install
cargo tauri dev
```

## 构建发布

在 GitHub Actions 手动触发 workflow，输入版本号即可。
