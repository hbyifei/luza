# 贡献指南

感谢你考虑为 Luza 做贡献！

## 许可证

Luza 使用 Apache License 2.0。所有贡献均在该许可证条款下提交。

## 开发者来源证书（DCO）

本项目使用 DCO（Developer Certificate of Origin），而非 CLA。
每位贡献者在提交时必须添加 `Signed-off-by` 行，表示你认证：

1. 你提交的代码是你本人创作，或你有权利按 Apache-2.0 提交
2. 你理解并同意，本项目及你的贡献是公开的，贡献记录将被永久保留

## 如何签署

在提交时添加 `-s` 参数：

    git commit -s -m "feat: 添加新功能"

提交信息会自动包含：

    Signed-off-by: 你的名字 <你的邮箱>

## 提交 PR 前请确认

- [ ] 代码已通过 `cargo check`
- [ ] 已添加必要的测试
- [ ] commit 已包含 `Signed-off-by`（通过 `git commit -s`）
- [ ] 已在 PR 描述中说明改动内容