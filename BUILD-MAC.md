# 怎么得到 Mac(M 芯片)版

**为什么不能在你这台 Windows 上打 Mac 版:** Mac 的 `.app/.dmg` 必须在 macOS 上构建——
它要用到 macOS 系统里的 WebKit(WKWebView)框架、`codesign`、`hdiutil` 这些只有 macOS 才有的东西,
从 Windows 交叉编译做不出能跑的 Mac 应用。所以 Mac 版有两条路:

---

## 路线 A:你手上有 Mac(M1/M2/M3…),最简单

在那台 Mac 上,把 `pdf-annotate-desktop` 整个文件夹拷过去,然后:

```bash
# 1) 一次性装环境
xcode-select --install                 # Xcode 命令行工具
# 装 Rust: https://rustup.rs  (curl ... | sh)
# 装 Node: https://nodejs.org 或 brew install node
rustup target add aarch64-apple-darwin

# 2) 打包
cd pdf-annotate-desktop
npm install
npm run tauri build -- --target aarch64-apple-darwin
```

出来的安装包在:
```
src-tauri/target/aarch64-apple-darwin/release/bundle/dmg/笺读_0.1.0_aarch64.dmg
```

> 没签名的话,Mac 第一次打开要"右键→打开",或在「系统设置 → 隐私与安全性」里点"仍要打开"。

---

## 路线 B:没有 Mac —— 用 GitHub Actions 在云端打(免费,推荐)

仓库里已经放好工作流 `.github/workflows/build.yml`,它会**同时**打 Windows 和 Mac M 芯片两个包。

步骤:

1. 在 GitHub 新建一个仓库(私有也行)。
2. 把 `pdf-annotate-desktop` 这个文件夹作为仓库根目录推上去:
   ```bash
   cd pdf-annotate-desktop
   git init && git add . && git commit -m "笺读 desktop"
   git branch -M main
   git remote add origin https://github.com/<你的用户名>/<仓库名>.git
   git push -u origin main
   ```
   (建议加一个 `.gitignore`,忽略 `node_modules/` 和 `src-tauri/target/`,别把编译产物推上去。)
3. 打开仓库的 **Actions** 标签页 → 选 `build-笺读` → 点 **Run workflow**。
4. 等几分钟,跑完在那次运行的 **Artifacts** 里下载:
   - `笺读-aarch64-apple-darwin` → 里面是 Mac M 芯片的 `.dmg`
   - `笺读-x86_64-pc-windows-msvc` → 里面是 Windows 的 `.exe`

> GitHub 公开仓库的 Actions 完全免费;私有仓库每月有免费额度(macOS 运行器较贵,偶尔打包够用)。

---

两条路产出的 Mac 包都是 **未签名/未公证** 的(没有 Apple 开发者账号 $99/年)。
个人或小范围用没问题,只是别人首次打开要手动允许一下。要做成"双击直接信任",得加 Apple 签名+公证,
那是另一套流程,需要时再说。
