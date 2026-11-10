# Open CAD Studio 轻量二维版

[English](../../README.md)

用于查看和编辑工程图纸的本地 CAD 工具，支持桌面端和浏览器。无需账户或云端转换服务，不包含广告、赞助入口、插件运行时或外部自动化服务。

## 保留功能

- 打开、编辑、恢复和保存 DWG、DXF、DWT、BAK、SV$ 文件
- PDF 与常见栅格图像的查看、附着和导入
- 直线、多段线、圆弧、圆、样条、区域和填充
- 文字、SHX 字体、尺寸标注、引线、表格和注释比例
- 图层、块、外部参照、模型空间和图纸空间
- 缩放、平移、测量、对象捕捉、属性查看和基础编辑
- SVG 导入，以及平台支持的 PDF、SVG、CSV 输出

本版已移除 3D 创作、外围三维模型格式、节点图、原生插件、MCP/REST 等外部自动化入口、更新检查和社交推广功能。

**识别范围：**现有 CAD 文字和 PDF 中的 SHX 文字可以显示、编辑或识别。本版没有新增扫描图片 OCR、AI 图元识别或自动算量。

## 启动 Web 页面

准备当前稳定版 Rust、Trunk 和 `wasm-bindgen-cli`，并安装 WebAssembly 目标：

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
```

在项目的 `OpenCADStudio` 目录运行：

```bash
trunk serve --release --offline --address 127.0.0.1 --port 8080
```

打开 <http://127.0.0.1:8080/app/>。图纸通过浏览器本地选择，保存时由你明确下载或写入，不需要上传到转换服务。

## 构建 Web 发布文件

```bash
trunk build --release
```

静态文件输出到 `dist/app/`，部署时需要保留 `/app/` 路径前缀，或相应调整 `Trunk.toml` 的 `public_url`。

## 启动桌面程序

开发模式：

```bash
cargo run --bin OpenCADStudio
```

直接打开一张或多张图纸：

```bash
cargo run --bin OpenCADStudio -- path/to/drawing.dwg
```

只读打开：

```bash
cargo run --bin OpenCADStudio -- --read-only path/to/drawing.dxf
```

构建优化的桌面可执行文件：

```bash
cargo build --release --bin OpenCADStudio
```

产物位于 `target/release/OpenCADStudio`；Windows 为 `OpenCADStudio.exe`。

## 验证

```bash
cargo check --workspace
cargo check --target wasm32-unknown-unknown -p OpenCADStudio -p ocs_web_worker
cargo test --workspace
```

请备份重要图纸。复杂第三方文件在投入生产使用前应先验证打开和另存结果。

## 许可证

项目使用 [GNU General Public License v3.0](../../LICENSE)。
