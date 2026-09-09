# Ed25519 签名密钥目录

本目录存放桌面端 V3 安全体系的签名验证公钥。

## 文件说明

| 文件 | 用途 | 是否入库 |
|------|------|----------|
| `ed25519-public.bin` | **编译期嵌入**（`catalog.rs` 的 `include_bytes!`），用于验证官网目录与自更新清单的 Ed25519 签名 | ✅ 必须入库（32 字节，公开材料，无泄露风险） |
| `ed25519-private-signing.bin` | 本地签名测试工具用（与官网部署私钥配对的裸 32 字节私钥） | ❌ gitignore（`ed25519-private*.bin`） |

## 密钥轮换流程

1. 生成新密钥对：
   ```bash
   # 任意 Node ≥ 18
   node -e "const {generateKeyPairSync}=require('crypto');const {writeFileSync}=require('fs');
   const {privateKey,publicKey}=generateKeyPairSync('ed25519');
   const pd=privateKey.export({type:'pkcs8',format:'der'});
   const qd=publicKey.export({type:'spki',format:'der'});
   writeFileSync('ed25519-private-signing.bin', pd.subarray(pd.length-32));
   writeFileSync('ed25519-public.bin', qd.subarray(qd.length-32));
   writeFileSync('../scripts/ed25519-private.pem', privateKey.export({type:'pkcs8',format:'pem'}));
   writeFileSync('../scripts/ed25519-public.pem', publicKey.export({type:'spki',format:'pem'}));"
   ```

2. 桌面端重新编译（`include_bytes!` 自动拾取新公钥）
3. 服务端部署：`scripts/ed25519-private.pem` 上传到服务器路径，环境变量
   `DSH_SIGNING_KEY_PATH` 指向该文件，重启服务
4. 更新 `scripts/key-info.json`（仅公钥信息，不含私钥）
5. git 提交 `ed25519-public.bin`（提交信息注明轮换日期）

## 安全边界

- **私钥绝不入库**：`pre-commit` 钩子（`scripts/git-hooks/`）会拦截 `ed25519-private*` / `key-info.json` 等敏感路径
- **公钥入库是构建必需**：fresh clone 后 `cargo build` 依赖 `include_bytes!("../keys/ed25519-public.bin")`；若该文件缺失，编译直接失败（fail-closed，而非静默跳过验签）
- 当前公钥（2026-09-08 第三次轮换）：`A2CCFF778846474BCABC31B4AB440714B275A8C0092DF7593D604E1056EBAB14`

## 历史

| 日期 | 事件 |
|------|------|
| 2026-09-03 | 第二次轮换（轮换时私钥被误提交到 archived 分支） |
| 2026-09-08 | 第三次轮换 + git filter-repo 清除历史私钥 blob（本目录私钥文件名从 `ed25519-private.bin` 改为 `ed25519-private-signing.bin`，明确其仅本地测试用途） |
