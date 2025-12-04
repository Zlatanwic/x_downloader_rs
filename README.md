# 一个基于Rust的X平台帖子视频下载工具
目前只有linux系统的可执行文件以及代码
## 配置方法
需要自行下载linux版本的yt-dlp可执行文件和ffmpeg可执行文件，放在本程序x_downloader_rs可执行文件的同目录下

并且需要你有自己的x账号，并在运行本程序前保持登录状态，并且在浏览器插件商店里找到“Get cookies.txt LOCALLY”插件，在x页面选择export，并且把输出的txt文件改成"cookies.txt"，也放在同目录下
## 用法
```shell
x_downloader_rs "url(替换为你想要的x帖子链接)"
```
## 保存
下载好的视频保存在download文件夹中
