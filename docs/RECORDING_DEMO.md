# Recording Demo Video Guide

This guide helps you record a compelling demo video for AutoResearchCodebaseWithHarness.

## 📹 What to Record

### Recommended: nanochat Optimization Demo

**Duration**: 2-3 minutes (sped up to show ~2 hour process)

**Script**:

1. **Opening (5 seconds)**
   ```bash
   # Terminal with clear prompt
   $ cd AutoResearchCodebaseWithHarness
   $ # Let's optimize Karpathy's nanochat overnight! 🚀
   ```

2. **Starting the optimization (10 seconds)**
   ```bash
   $ cargo run --example nanochat_optimization
   
   🚀 AutoResearchCodebaseWithHarness × nanochat
   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   Optimizing Karpathy's nanochat training hyperparameters
   
   📦 Cloning nanochat repository...
   ✓ nanochat cloned
   ```

3. **Baseline run (15 seconds)**
   ```
   📊 Running baseline configuration...
      (batch_size=64, lr=3e-4, model=124M)
   
   ✓ Baseline Results:
      val_bpb:       2.8347
      peak_vram:     18.4 GB
      throughput:    487.2M tokens
   ```

4. **Experiments montage (60 seconds - sped up)**
   Show experiments running with:
   - ✅ KEPT markers (green)
   - ❌ DISCARDED markers (red)
   - Progress bar filling up
   - Time lapse effect (2 hours → 60 seconds)
   
   ```
   🔬 Running 15 hyperparameter experiments...
   
   Experiment 1/15: batch_size=128, lr=5e-4
      ✅ KEPT: val_bpb 2.8347 → 2.7892
   
   Experiment 2/15: batch_size=32, lr=1e-4
      ❌ DISCARDED: val_bpb 2.8512 (+0.0165)
   
   [Progress bar animation]
   [■■■■■■■■■■■■■■■] 15/15 complete
   ```

5. **Final results (30 seconds)**
   ```
   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   🏆 OPTIMIZATION COMPLETE
   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   
   📊 Baseline: val_bpb 2.8347, VRAM 18.4GB
   🎯 Best:     val_bpb 2.7534 ✨ (2.87% better)
                VRAM 18.6GB, throughput +5.3%
   
   💰 Saves $140/month in training costs
   📈 Success rate: 26.7% (4 kept, 11 discarded)
   
   Ready for production! 🚀
   ```

6. **Closing (10 seconds)**
   ```
   # Show the saved config file
   $ cat experiments/nanochat_optimization/best_config.json
   {
     "batch_size": 64,
     "learning_rate": 1e-3,
     "model": "124M"
   }
   
   # End with project URL
   github.com/bianbiandashen/AutoResearchCodebaseWithHarness
   ```

## 🛠️ Recording Tools

### Option 1: asciinema (Recommended for Terminal)

**Best for**: Terminal-based demos, easy sharing, small file size

```bash
# Install
brew install asciinema  # macOS
apt install asciinema   # Linux

# Record
asciinema rec demo.cast

# Convert to GIF
npm install -g asciicast2gif
asciicast2gif demo.cast demo.gif

# Convert to MP4
docker run --rm -v $PWD:/data asciinema/asciicast2gif \
  demo.cast demo.gif && \
  ffmpeg -i demo.gif -movflags faststart \
  -pix_fmt yuv420p -vf "scale=trunc(iw/2)*2:trunc(ih/2)*2" demo.mp4
```

### Option 2: Screen Recording (macOS)

```bash
# QuickTime Player (built-in)
# 1. Open QuickTime Player
# 2. File → New Screen Recording
# 3. Select terminal window
# 4. Record the demo following the script

# Compress for web
ffmpeg -i recording.mov -vcodec h264 -acodec aac demo.mp4
```

### Option 3: OBS Studio (Cross-platform)

**Best for**: High-quality recordings with annotations

```bash
# Download: https://obsproject.com/

# Settings:
# - Resolution: 1920x1080 or 1280x720
# - FPS: 30
# - Encoder: x264
# - Bitrate: 2500 Kbps
```

### Option 4: terminalizer (Animated GIF)

```bash
# Install
npm install -g terminalizer

# Record
terminalizer record demo

# Render GIF
terminalizer render demo -o demo.gif

# Convert to MP4
ffmpeg -i demo.gif -movflags faststart \
  -pix_fmt yuv420p -vf "scale=trunc(iw/2)*2:trunc(ih/2)*2" demo.mp4
```

## 🎨 Recording Tips

### Terminal Setup

```bash
# Use a clean, readable theme
# Recommended: iTerm2 with Dracula theme or VS Code terminal

# Increase font size for visibility
# Recommended: 16-18pt (recording) → appears as 12-14pt (video)

# Clear screen before recording
clear

# Set a simple PS1 prompt
export PS1="$ "

# Increase terminal size
# Recommended: 120 columns × 30 rows
```

### Speed Up Long Operations

```bash
# For the experiments montage, record normally then speed up in post:
ffmpeg -i input.mp4 -filter:v "setpts=0.1*PTS" -an output-fast.mp4

# Or use asciinema's speed option:
asciinema play -s 3 demo.cast  # 3× speed
```

### Add Annotations (Optional)

Use tools like:
- **Keynote/PowerPoint**: Export as video with callouts
- **DaVinci Resolve**: Free professional video editor
- **Camtasia**: Professional screen recording with annotations

## 📤 Publishing the Video

### GitHub (Recommended)

```bash
# 1. Upload to GitHub Release
# Go to: https://github.com/bianbiandashen/AutoResearchCodebaseWithHarness/releases
# Create new release v0.1.0
# Upload demo.mp4

# 2. Get the asset URL
# It will be: https://github.com/bianbiandashen/AutoResearchCodebaseWithHarness/releases/download/v0.1.0/demo.mp4

# 3. Update README.md
# Replace placeholder with actual URL
```

### YouTube (Alternative)

```bash
# 1. Upload to YouTube
# 2. Get embed code or link
# 3. Update README with YouTube link

# Example markdown:
[![Demo Video](https://img.youtube.com/vi/VIDEO_ID/maxresdefault.jpg)](https://www.youtube.com/watch?v=VIDEO_ID)
```

### Bilibili (For Chinese Audience)

```bash
# 1. Upload to Bilibili
# 2. Get BV number
# 3. Update README_CN.md

# Example:
[观看演示](https://www.bilibili.com/video/BV1234567890)
```

### Self-hosted (GitHub Assets)

```bash
# Use GitHub's built-in video hosting
# Just drag and drop .mp4 into a GitHub comment or issue
# Copy the generated URL
```

## 🎬 Alternative: Animated GIF

If video hosting is difficult, use an animated GIF:

```bash
# Record and convert to optimized GIF
asciinema rec demo.cast
asciicast2gif demo.cast demo.gif

# Optimize file size
gifsicle -O3 --colors 256 demo.gif -o demo-optimized.gif

# Add to README
![Demo](./docs/demo.gif)
```

## ✅ Final Checklist

Before publishing:

- [ ] Video is 2-3 minutes long
- [ ] Terminal font size is readable (16-18pt)
- [ ] All text is clearly visible
- [ ] Speed-up sections are smooth (no jarring jumps)
- [ ] Audio is clear (if included) or muted
- [ ] File size < 50MB (for GitHub)
- [ ] Video format: MP4 (H.264 codec, YUV420p pixel format)
- [ ] Tested on both desktop and mobile
- [ ] Works in both light and dark mode (if applicable)
- [ ] Fallback links provided (YouTube/Bilibili)
- [ ] Thumbnail is attractive (first frame or custom)

## 📝 Video Description Template

Use this for YouTube/Bilibili description:

```
AutoResearchCodebaseWithHarness - Autonomous AI Research Framework

Watch as we optimize Karpathy's nanochat LLM training hyperparameters overnight!

✨ What happens:
• Framework tests 15 different configurations automatically
• Keeps improvements (better val_bpb or lower VRAM)
• Discards regressions (worse performance)
• Finds optimal settings: 2.87% better + $140/month savings

⏱️ Timeline:
0:00 - Introduction
0:10 - Starting optimization
0:25 - Baseline run
0:40 - Experiments montage (15 tests, sped up)
1:40 - Final results
2:10 - Cost savings breakdown

🔗 Links:
GitHub: https://github.com/bianbiandashen/AutoResearchCodebaseWithHarness
Docs: https://github.com/bianbiandashen/AutoResearchCodebaseWithHarness#readme

#AI #MachineLearning #LLM #Rust #Automation #nanochat #Karpathy
```

## 🆘 Troubleshooting

**Video too large?**
```bash
# Compress with ffmpeg
ffmpeg -i input.mp4 -vcodec h264 -crf 28 output.mp4
```

**Low quality?**
```bash
# Increase bitrate
ffmpeg -i input.mp4 -b:v 5M output.mp4
```

**Can't upload to GitHub?**
```bash
# Split into chunks
ffmpeg -i demo.mp4 -t 00:01:30 -c copy part1.mp4
ffmpeg -i demo.mp4 -ss 00:01:30 -c copy part2.mp4
```

## 🎯 Success Criteria

A good demo video should:

1. ✅ Show the complete workflow start-to-finish
2. ✅ Be understandable without audio (captions/text)
3. ✅ Demonstrate real-world value ($140/month savings)
4. ✅ Load quickly (< 50MB)
5. ✅ Work on mobile devices
6. ✅ Make viewers want to try it immediately

---

**Need help?** Open an issue: https://github.com/bianbiandashen/AutoResearchCodebaseWithHarness/issues
