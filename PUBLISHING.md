# Open Snapora 发布维护指南 (Internal Publishing Guide)

> ⚠️ 本文档仅供项目内部维护者参考，用于指导 NPM 包与 Rust 插件的发布流程。请勿将内部操作细节写入面向外部使用者的对外 `README.md`。

---

## 一、前置准备与账号验证

### 1. npm 官方源登录与身份检查

在发布前，必须确保本地终端已在 npm 官方注册源（而非国内只读镜像）完成认证：

```bash
# 1. 登录 npm 官方源（按提示在浏览器或终端完成密码、邮箱及 OTP 验证）
npm login --registry=https://registry.npmjs.org/

# 2. 检查当前已登录的用户名
npm whoami --registry=https://registry.npmjs.org/
```

### 2. npm 组织 (Scope) 权限确认

本项目发布的包均带有 `@open-snapora/` 作用域：
- 确保 [npmjs.com](https://www.npmjs.com) 上已创建 `open-snapora` Organization；
- 确保当前登录的 npm 账号是该组织的成员且具备**包发布权限 (Developer / Admin)**。

---

## 二、发包前全量校验与模拟预检

在正式发包前，必须执行全量类型检查、代码构建与 Dry-Run 预检：

```bash
# 1. 全量校验：类型检查、包构建、Rust 语法与演示工程编译
pnpm run check:all

# 2. 模拟发布（Dry Run）：验证依赖拓扑排序与打包清单，不实际推送到 npm
pnpm run publish:dry-run
```

确认终端输出中 4 个包均标记为 `(dry run)` 且无任何报错：
- `📦 @open-snapora/shared@x.x.x`
- `📦 @open-snapora/electron@x.x.x`
- `📦 @open-snapora/overlay@x.x.x`
- `📦 @open-snapora/tauri@x.x.x`

---

## 三、执行正式 NPM 发布

确认预检无误后，在项目根目录运行一键发布命令：

```bash
pnpm run publish:packages
```

> **自动化机制说明**：
> - pnpm 会自动按照拓扑顺序依次发布底层包与上层包；
> - 自动执行各个子包的 `prepublishOnly` 构建最新 `dist/`；
> - 自动将各包 `package.json` 中的 `workspace:*` 解析替换为实际语义版本号；
> - 自动指定 `--registry=https://registry.npmjs.org/` 与 `--access public`。

---

## 四、Rust 插件配套发布 (`tauri-plugin-snapora`)

针对 Tauri 用户使用的底层 Rust 插件，提供两种维护方式：

### 方式 A：Git Tag 引用发布（最推荐）

当推送代码并发布 NPM 后，为对应版本打上 Git 标签并推送到 GitHub：

```bash
# 打上版本标签
git tag v0.1.0
git push origin v0.1.0
```

Tauri 用户即可通过 `Cargo.toml` 稳定拉取：
```toml
tauri-plugin-snapora = { git = "https://github.com/open-toolkits/open-snapora.git", tag = "v0.1.0" }
```

### 方式 B：crates.io 官方发布

若需发布至 Rust 官方 crates.io：

```bash
# 1. 本地登录 crates.io（需 API Token）
cargo login

# 2. 发布 Rust 插件包
cargo publish --manifest-path crates/tauri-plugin-snapora/Cargo.toml
```

---

## 五、版本升级指南 (Version Bump)

发布新版本（如 `0.2.0`）时：

1. 更新各子包 `packages/*/package.json` 及 `crates/tauri-plugin-snapora/Cargo.toml` 中的 `version` 字段；
2. 运行 `pnpm run build` 重新编译；
3. 提交 Git commit，并按照上述流程重新执行 `publish:dry-run` 与 `publish:packages`。
