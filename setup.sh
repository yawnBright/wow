#!/bin/bash

# 项目安装路径
proj_path="$HOME/Pictures/wow"
# 可执行文件下载地址
# bin_url="https://gitee.com/yawnbright/pub_space/releases/download/wall_paper_test/wow"
bin_url="https://gitee.com/yawnbright/pub_space/releases/download/wall_paper_test2.0/wow"
# 可执行文件名
bin_name="wow"
# 可执行文件保存路径
bin_save_path=$proj_path
# 可执行文件路径
bin="$bin_save_path/$bin_name"
# 图片保存路径
img_save_path="$proj_path/image"

mkdir -p "$proj_path"
mkdir -p "$bin_save_path"
mkdir -p "$img_save_path"

if command -v curl >/dev/null 2>&1; then
  curl -L $bin_url -o "$bin"
  chmod +x "$bin"
else
  echo "请先安装curl"
  exit
fi

# 定义新的定时任务
NEW_CRON="*/1 * * * * $bin" # 每1分钟执行一次

# 临时文件存储现有crontab
TEMP_CRON=$(mktemp)

# 保存现有crontab（如果有）
crontab -l >"$TEMP_CRON" 2>/dev/null

# 检查任务是否已存在，避免重复添加
if ! grep -q "$NEW_CRON" "$TEMP_CRON"; then
  echo "$NEW_CRON" >>"$TEMP_CRON"
  crontab "$TEMP_CRON"
  echo "已添加定时任务：$NEW_CRON"
else
  echo "任务已存在，跳过添加"
fi

# 清理临时文件
rm "$TEMP_CRON"
