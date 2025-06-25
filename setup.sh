#!/bin/bash

# 项目安装路径
proj_path="$HOME/Pictures/wow"

# 可执行文件下载地址
bin_url="https://gitee.com/yawnbright/wow/releases/download/v1.0/wow-mac-arm64"
bin_updater_url="https://gitee.com/yawnbright/wow/releases/download/v1.0/updater-mac-arm64"

# 可执行文件名
bin_name="wow"
updater_name="updater"

# 可执行文件保存路径
bin_save_path=$proj_path

# 可执行文件路径
bin="$bin_save_path/$bin_name"
updater="$bin_save_path/$updater_name"

# 图片保存路径
img_save_path="$proj_path/"

mkdir -p "$proj_path"
mkdir -p "$bin_save_path"
mkdir -p "$img_save_path"

if command -v curl >/dev/null 2>&1; then
  curl -L "$bin_url" -o "$bin"
  curl -L "$bin_updater_url" -o "$updater"

  chmod +x "$bin"
  chmod +x "$updater"
else
  echo "请先安装curl"
  exit
fi

# 添加环境变量
CONFIG_FILE="$HOME/.bashrc"
if [[ "$SHELL" == *"zsh"* ]]; then
  CONFIG_FILE="$HOME/.zshrc"
fi

touch "$CONFIG_FILE"

echo "export PATH=\"$proj_path:\$PATH\"" >>"$CONFIG_FILE"

echo ""
echo "安装路径: $proj_path"
echo "使用：\`source $CONFIG_FILE\` 来激活环境变量"

touch "$proj_path/wow-run"
echo "nohup $bin run >/dev/null 2>&1 &" >"$proj_path/wow-run"
chmod +x "$proj_path/wow-run"

echo "使用\`wow-run\`开启自动更新"
