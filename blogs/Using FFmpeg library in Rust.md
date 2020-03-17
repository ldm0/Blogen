Using FFmpeg library in Rust   
2020/3/18  
Rust | C | Programming | English  

---

Use the inner library of FFmpeg in Rust through FFI.

---

> Due to the boring school lessons, I decided to participate in GSoC 2020 this year. Since I worked on some Rust projects before, the `FFmpeg + Rust: Code builder` GSoC project really caught me. Then I started to do some pre-work. The pre-work consists of several parts and I will talk about the FFI part in this blog post.

## Intro
FFmpeg project not only provides us ffmpeg, ffplay and ffprobe. They also packed their inner modules for us which can be easily used(relatively). FFmpeg is written in C and that's very convenient for FFI.

## Coding
First, we need some application code which uses the library. Though I have looked into the libavfilter, I'm not quite familiar with the other ffmpeg interfaces. So I searched for some c code that uses the ffmpeg inner libraries, then I got [this](https://github.com/leandromoreira/ffmpeg-libav-tutorial). This is a FFmpeg library using tutorial, which is actually pretty good. I choose the [first demo](https://github.com/leandromoreira/ffmpeg-libav-tutorial/blob/master/0_hello_world.c) to port it into Rust.

## Porting
This demo uses libavcodec and libavformat. To get proper linting when language porting, function and data structures declaration is needed. So I start to translate the header file into Rust. That's an impossible mission. 5 minutes later I realized that's impossible. Luckily I found the [bingen](https://github.com/rust-lang/rust-bindgen). It parses the C header file and automatically generates the corresponding Rust expression. The only problem is that C macros are not translated, we need to manually convert them(not quite hard though). With the generated interfaces, I successfully ported the demo into Rust after some fierce fight with unsafe and type inference lol. The only thing make me sad is that the original demo code has some bug which lead to segment fault and took me several hours to figured out that's not my problem... I sent them a [pull request](https://github.com/leandromoreira/ffmpeg-libav-tutorial/pull/60) to fix it. Souce code is [here](https://github.com/ldm0/no_name/blob/master/src/main.rs).

## Building
The hardest part is actually building it. My first FFmpeg build is on Windows using msvc(debug a complex codebase with Visual Studio is enjoyable). However, object file generated with clang cannot be linked with msvc lib file. So I migrated to Linux and built FFmpeg libraries with Linux toolchain. The dependencies use in the demo code building process is miscellaneous. I started by finding corresponding dependencies with unresolved external function names and that's also a impossible mission. Then I found that in FFmpeg project the `ffbuild/config.mak` file generated with the `configure` script contains compiler parameters for the each library. After turning the compiler parameters to Rust build script, the demo works!

```
initializing all the containers, codecs and protocols.
opening the input file (bear.mp4) and loading format (container) header
format mov,mp4,m4a,3gp,3g2,mj2, duration 1066667 us, bit_rate 0
finding stream info from format
AVStream->time_base before open coded 1/44100
AVStream->r_frame_rate before open coded 0/0
AVStream->start_time 1024
AVStream->duration 47104
finding the proper decoder (CODEC)
Audio Codec: 2 channels, sample rate 44100
        Codec aac ID 86018 bit_rate 74553
AVStream->time_base before open coded 1/30000
AVStream->r_frame_rate before open coded 30000/1001
AVStream->start_time 1001
AVStream->duration 30030
finding the proper decoder (CODEC)
Video Codec: resolution 320 x 180
        Codec h264 ID 27 bit_rate 233574
AVPacket->pts 1001
AVPacket->pts 3003
Frame 1 (type=73, size=6355 bytes) pts 1001 key_frame 1 [DTS 0]
AVPacket->pts 2002
Frame 2 (type=66, size=216 bytes) pts 2002 key_frame 0 [DTS 2]
AVPacket->pts 5005
Frame 3 (type=80, size=1028 bytes) pts 3003 key_frame 0 [DTS 1]
AVPacket->pts 4004
Frame 4 (type=66, size=329 bytes) pts 4004 key_frame 0 [DTS 4]
AVPacket->pts 7007
Frame 5 (type=80, size=1223 bytes) pts 5005 key_frame 0 [DTS 3]
AVPacket->pts 6006
Frame 6 (type=66, size=260 bytes) pts 6006 key_frame 0 [DTS 6]
AVPacket->pts 9009
Frame 7 (type=80, size=1160 bytes) pts 7007 key_frame 0 [DTS 5]
releasing all the resources
```