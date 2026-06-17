# 批注台 · AI 精读 —— 桌面版(Tauri)

把原来的网页版包成一个能双击打开的 Windows 软件，数据全部存进你自己选的本地文件夹，
不再压在浏览器里。前端用的就是修好图表精读 bug 的那份 `index.html`，
只换掉了存储那一层。

---

## 数据存在哪

第一次打开会让你**选一个文件夹**当文献库。之后所有东西都落在里面：

```
你选的文件夹/
├── library.json                  章节清单(标题、顺序、标签)
├── config.json                   API 配置(便于换电脑带走)
└── papers/
    └── <每篇一个 id>/
        ├── source.pdf            原始 PDF
        ├── annotations.json      批注 / 全文总览 / 图表精读 / 问题 / 大纲
        └── supplements.json      支撑材料(纯文本)
```

好处很直接：

- `annotations.json` 是纯文本，记事本就能打开看。
- 整个文件夹丢进 OneDrive / 坚果云 / 移动硬盘 / git，就完成了备份和多机同步。
- PDF 是普通文件，不受浏览器存储配额限制，存几百篇也不怕。

想换文件夹：左上角有个 **📁 文件夹** 按钮，点一下重选，重选后会刷新。
路径记在本机，下次自动用上次那个。

---

## 一次性装环境(只做一遍)

桌面版要编译一段 Rust 后端，所以比网页版多装两样东西。按顺序来：

**1. Microsoft C++ 生成工具**（Rust 在 Windows 上链接要用）
- 下载 “Build Tools for Visual Studio”，安装时勾选 **“使用 C++ 的桌面开发”** 这个工作负载。

**2. Rust**（用 rustup 装，顺带选 MSVC 工具链)
```powershell
winget install --id Rustlang.Rustup
rustup default stable-msvc
```

**3. Node.js**：你已经有了（仅用来跑 Tauri 的命令行）。

**4. WebView2**：Win11 自带，不用管。

装完关掉再重开一个终端，确认这几条都有输出：
```bash
rustc --version
cargo --version
node --version
```

---

## 跑起来 / 打包

在本文件夹（`pdf-annotate-desktop`）里：

```bash
# 1) 装 Tauri 命令行
npm install

# 2) 生成应用图标(随便给一张 1024×1024 的方形 PNG;打包前必须做一次)
npm run tauri icon ./app-icon.png

# 3) 开发模式:开个窗口实时跑(第一次会编译 Rust,要等几分钟)
npm run tauri dev

# 4) 打成安装包
npm run tauri build
```

打包好的安装程序在：
```
src-tauri/target/release/bundle/nsis/批注台_0.1.0_x64-setup.exe
```
双击安装，就是一个正常的 Windows 软件了。

> 第一次 `tauri dev` / `tauri build` 会下载并编译一堆 Rust 依赖，慢是正常的，之后就快了。

---

## 以后怎么升级前端

前端（界面 + 逻辑）就是 `ui/index.html` 这一个文件。
以后网页版再改了功能，把新的 HTML 覆盖到 `ui/index.html`、重新 `npm run tauri build` 就行——
**唯一要保留的**是文件顶部那段 `STORAGE`（Tauri 本地文件夹存储）和它对 `idbPut/idbGet/idbAll/idbDel`、
`openDB`、保存设置、启动那几处的改动。换句话说：在新网页版上重新打这几个补丁，或者反过来，把界面改动并进这份带补丁的文件。

---

## 这一版的边界(说在前面，免得你以为坏了)

1. **CORS 还在。** 桌面版里普通 `fetch()` 仍然受浏览器同源策略管，所以非 Claude 的模型
   (Kimi / DeepSeek / MiMo) 可能还是会被 CORS 挡。**要彻底解决，得把 API 调用从前端 `fetch`
   改成走 Rust 发请求**（加一个 `http_request` 命令 + `reqwest`，流式输出再用 Channel 推回前端)。
   这是清楚的下一步，没在这一版里做——告诉我就加。

2. **少量界面偏好仍在应用本地存储里**：文件夹分组、自定义标签、文献广场缓存这些小东西，
   目前还存在应用自己的本地区，没进你选的文件夹。核心数据（PDF + 批注 + 清单 + API 配置）
   都已经在文件夹里了。要把这些也搬进文件夹，是个小活，需要的话一起做。

3. **大文件(几十兆以上的论文)** 读取走的是 base64，理论上略慢。常规论文(几兆到二十兆)
   完全无感。真嫌慢，可以把读取改成 Tauri 的二进制通道(`tauri::ipc::Response`)，也是后续优化。

---

## 一个容易踩的坑(给以后改 Rust 的你)

Tauri v2 默认会把 Rust 端的 `snake_case` 参数名转成前端的 `camelCase`。
这份代码里所有命令参数都故意用了单字（`path` / `contents` / `b64`），就是为了绕开这个转换——
你要是新加带下划线的参数名（比如 `dir_path`），前端调用时得写成 `dirPath`，否则参数会变成 `undefined`。
