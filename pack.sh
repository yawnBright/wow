#!/bin/bash

# 构建脚本，用于打包应用程序

# 配置参数
OUTPUT="./pack"
BINARY_NAME="wow"
UPDATER_NAME="updater"
UPDATER_SOURCE="./updater/mac/main.swift"
UPDATER_OUTPUT="./updater/mac/updater-mac-arm64"

# 错误处理函数
handle_error() {
  echo "错误: 在 $1 步骤失败"
  exit 1
}

# 检查依赖
check_dependencies() {
  echo "检查依赖..."
  command -v cargo >/dev/null 2>&1 || {
    echo >&2 "需要安装cargo (Rust)"
    exit 1
  }
  command -v swiftc >/dev/null 2>&1 || {
    echo >&2 "需要安装Swift编译器"
    exit 1
  }
  echo "依赖检查通过"
}

# 清理输出目录
clean_output() {
  echo "清理输出目录..."
  mkdir -p "${OUTPUT:?}" || handle_error "创建输出目录"
  # 使用${OUTPUT:?}确保变量不为空，防止扩展为/*
  rm -rf "${OUTPUT:?}"/* || handle_error "清理输出目录"
  echo "输出目录清理完成"
}

# 构建Rust应用
build_rust_app() {
  echo "开始构建Rust应用..."
  cargo build --release || handle_error "Rust应用构建"
  if [ ! -f "./target/release/$BINARY_NAME" ]; then
    handle_error "找不到编译后的Rust二进制文件"
  fi
  echo "Rust应用构建完成"
}

# 构建Swift更新器
build_swift_updater() {
  echo "开始构建Swift更新器..."
  if [ ! -f "$UPDATER_SOURCE" ]; then
    handle_error "找不到更新器源代码"
  fi
  swiftc "$UPDATER_SOURCE" -o "$UPDATER_OUTPUT" || handle_error "Swift更新器构建"
  if [ ! -f "$UPDATER_OUTPUT" ]; then
    handle_error "找不到编译后的更新器"
  fi
  echo "Swift更新器构建完成"
}

# 复制文件到输出目录
copy_files() {
  echo "复制文件到输出目录..."
  cp "./target/release/$BINARY_NAME" "${OUTPUT:?}/" || handle_error "复制Rust二进制文件"
  cp "$UPDATER_OUTPUT" "${OUTPUT:?}/$UPDATER_NAME" || handle_error "复制更新器"
  echo "文件复制完成"
}

# 主函数
main() {
  check_dependencies
  clean_output
  build_rust_app
  build_swift_updater
  copy_files
  echo "打包完成! 文件已输出到 $OUTPUT 目录"
}

# 执行主函数
main
