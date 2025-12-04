use std::process::{Command, Stdio};
use std::path::Path;
use anyhow::{Context, Result};
// use std::io::{self, Write};
use std::env;

fn main() -> Result<()> {
    
    let args: Vec<String> = env::args().collect();
    
    
    if args.len() != 2 {
        eprintln!("错误: 未提供 URL");
        
        eprintln!("用法: {} <X帖子链接>", args[0]); 
        std::process::exit(1);
    }

    
    let url = &args[1];
    let cookie_file = "cookies.txt";
    
    let output_format = "download/%(uploader_id)s_%(id)s.%(ext)s";

    println!("--------------------------------------------------");
    println!("   🚀 X (Twitter) Video Downloader - CLI Mode");
    println!("--------------------------------------------------");

    
    let ytdlp_bin = if cfg!(target_os = "windows") {
        "yt-dlp.exe"
    } else {
        "./yt-dlp" 
    };
    
    
    if !Path::new(ytdlp_bin).exists() {
        eprintln!("严重错误: 当前目录下找不到 {}", ytdlp_bin);
        eprintln!("请确保 yt-dlp 二进制文件与本程序在同一目录下。");
        std::process::exit(1);
    }

    
    if !Path::new(cookie_file).exists() {
        return Err(anyhow::anyhow!("错误: 找不到 '{}'！请先导出 Cookies 并放入该目录。", cookie_file));
    }

    
    let ffmpeg_bin = if cfg!(target_os = "windows") { "ffmpeg.exe" } else { "ffmpeg" };
    let has_local_ffmpeg = Path::new(ffmpeg_bin).exists();

    
    println!("目标链接: {}", url);
    println!("正在启动下载引擎...\n");
    
    let mut cmd = Command::new(ytdlp_bin);
    
    
    cmd.arg("--cookies")
       .arg(cookie_file)
       .arg("-o")
       .arg(output_format)
       .arg("-f")
       .arg("bestvideo+bestaudio/best"); // 最佳画质

    // 如果当前目录有 ffmpeg，就强制使用当前目录的；否则让 yt-dlp 去系统环境找
    if has_local_ffmpeg {
        cmd.arg("--ffmpeg-location").arg("./");
    } else {
        println!("提示: 当前目录未找到 {}, 将尝试使用系统安装的 FFmpeg。", ffmpeg_bin);
    }

    
    let status = cmd.arg(url)
        .stdout(Stdio::inherit()) // 把工具的输出直接打印到终端
        .stderr(Stdio::inherit())
        .status()
        .context("无法启动 yt-dlp 进程")?;

    
    if status.success() {
        println!("\n下载成功！视频已保存到 'download' 文件夹。");
    } else {
        eprintln!("\n下载失败。请检查上方日志 (可能是网络问题或 Cookie 过期)。");
    }

    Ok(())
}