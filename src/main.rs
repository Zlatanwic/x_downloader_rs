use std::process::{Command, Stdio};
use std::path::Path;
use anyhow::{Context, Result};
// use std::io::{self, Write};
use std::env;

fn main() -> Result<()> {
    // 1. 获取命令行参数
    let args: Vec<String> = env::args().collect();
    
    // 如果没有提供 URL，打印用法并退出
    if args.len() != 2 {
        eprintln!("❌ 错误: 未提供 URL");
        // args[0] 是程序本身的路径/名字
        eprintln!("💡 用法: {} <X帖子链接>", args[0]); 
        std::process::exit(1);
    }

    // 获取 URL
    let url = &args[1];
    let cookie_file = "cookies.txt";
    // 视频将保存到 download 文件夹中（yt-dlp 会自动创建该文件夹）
    let output_format = "download/%(uploader_id)s_%(id)s.%(ext)s";

    println!("--------------------------------------------------");
    println!("   🚀 X (Twitter) Video Downloader - CLI Mode");
    println!("--------------------------------------------------");

    // 2. 确定 yt-dlp 的文件名 (Windows vs Linux)
    let ytdlp_bin = if cfg!(target_os = "windows") {
        "yt-dlp.exe"
    } else {
        "./yt-dlp" // Linux 必须加 ./
    };
    
    // 检查 yt-dlp 是否存在
    if !Path::new(ytdlp_bin).exists() {
        eprintln!("⚠️  严重错误: 当前目录下找不到 {}", ytdlp_bin);
        eprintln!("请确保 yt-dlp 二进制文件与本程序在同一目录下。");
        std::process::exit(1);
    }

    // 检查 cookies.txt 是否存在
    if !Path::new(cookie_file).exists() {
        return Err(anyhow::anyhow!("❌ 错误: 找不到 '{}'！请先导出 Cookies 并放入该目录。", cookie_file));
    }

    // 3. 确定 ffmpeg 的文件名 (仅用于检查存在性)
    let ffmpeg_bin = if cfg!(target_os = "windows") { "ffmpeg.exe" } else { "ffmpeg" };
    let has_local_ffmpeg = Path::new(ffmpeg_bin).exists();

    // 4. 构建并执行命令
    println!("目标链接: {}", url);
    println!("正在启动下载引擎...\n");
    
    let mut cmd = Command::new(ytdlp_bin);
    
    // 添加基础参数
    cmd.arg("--cookies")
       .arg(cookie_file)
       .arg("-o")
       .arg(output_format)
       .arg("-f")
       .arg("bestvideo+bestaudio/best"); // 最佳画质

    // 【智能判断】如果当前目录有 ffmpeg，就强制使用当前目录的；否则让 yt-dlp 去系统环境找
    if has_local_ffmpeg {
        cmd.arg("--ffmpeg-location").arg("./");
    } else {
        println!("⚠️  提示: 当前目录未找到 {}, 将尝试使用系统安装的 FFmpeg。", ffmpeg_bin);
    }

    // 添加 URL 并配置输出
    let status = cmd.arg(url)
        .stdout(Stdio::inherit()) // 把工具的输出直接打印到终端
        .stderr(Stdio::inherit())
        .status()
        .context("无法启动 yt-dlp 进程")?;

    // 5. 检查结果
    if status.success() {
        println!("\n✅ 下载成功！视频已保存到 'download' 文件夹。");
    } else {
        eprintln!("\n❌ 下载失败。请检查上方日志 (可能是网络问题或 Cookie 过期)。");
    }

    Ok(())
}