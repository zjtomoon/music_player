# 终端音乐播放器——基于rust tui框架实现

## cargo工程依赖

+ `rodio`

+ `audiotags`

+ `mp3-duration`  用于显示音乐的时长

+ `exitfailure` 

+ `dirs`

+ `serde`

+ `serde-yaml`

+ `crossterm`

+ `tui` 终端ui框架

+ `infer`

+ `regex`

+ `lazy_static`

+ `rand`

## rust版本

```toml
version = "2021"
```

## cargo配置

```toml
[package]
name = "music_player"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rodio = "0.15.0"
audiotags = "0.2"
mp3-duration = "0.1.10"
exitfailure = "0.5.1"
dirs = "4.0.0"
serde = {version = "1",features = ["derive"]}
serde_yaml = "0.8"
crossterm = "0.22.1"
tui = "0.17.0"
infer = "0.6.0"
regex = "1.5"
lazy_static = "1.4.0"
rand = "0.8"
```

## 配置

```bash
mkdir ~/.config/music_player/config.yml
```

配置文件是目录中的config.yml

## 使用

### 快捷键

| Description                  | Event                  |
| ---------------------------- | ---------------------- |
| Exit program                 | q                      |
| Move selection down          | j \| \<Down Arrow Key> |
| Move selection up            | k \| \<Up Arrow Key>   |
| Move selection down 5 steps  | J                      |
| Move selection up 5 steps    | K                      |
| Move selection to the top    | g                      |
| Move selection to the bottom | G                      |
| Next page                    | n                      |
| Previous page                | N                      |
| Open folder                  | l                      |
| Back to the previous folder  | h                      |
| Enter command mode           | :                      |
| Enter search mode            | \|                     |
| Exit search or command mode  | \<Esc>                 |
| Pause or resume              | \<Space>               |
| Increase the volume          | + \| =                 |
| Decrease the volume          | -                      |
| Add music to the playlist    | \<Enter>               |

### 命令

| Description                                                              | Command           |
| ------------------------------------------------------------------------ | ----------------- |
| Add all songs in the current directory to the playlist                   | all               |
| Removes the specified song from the playlist (Multiple can be specified) | rm \<music_id>    |
| Remove all songs from the playlist                                       | clear \| cls      |
| Play the whole playlist (Repeat: on)                                     | order \| od       |
| Put the current song on repeat (Repeat: off)                             | singlecycle \| sc |
| Shuffle current playlist                                                 | shuffle \| sh     |
| Play the next song in the playlist                                       | next \| n         |
