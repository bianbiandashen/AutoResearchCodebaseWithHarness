# Video Optimization Skill

Automatically find the optimal video encoding parameters to achieve the best quality-to-size ratio.

## What It Does

This skill runs systematic experiments on video encoding parameters:
- **Codecs**: H.264, H.265 (HEVC), AV1
- **CRF Values**: 18-28 (quality levels)
- **Presets**: slow, medium, fast (encoding speed)
- **Resolutions**: 1080p, 720p, 480p

For each combination, it measures:
- **VMAF Score**: Perceptual video quality (0-100, higher is better)
- **File Size**: Output file size in MB
- **Encoding Time**: How long it took to encode
- **Bitrate**: Average bitrate in kbps

## How to Use

### 1. Basic Usage

```bash
# Place your input video in the project directory
cp my_video.mp4 input.mp4

# Run the optimization skill
autocodeharness run-skill video-optimization

# Wait for results (15 experiments × ~1 min each = ~15 minutes)
```

### 2. With Rust API

```rust
use autocodeharness::{
    skills::load_skill,
    harness::Harness,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load the skill
    let skill = load_skill("video-optimization")?;
    
    // Configure input
    skill.set_input("video_path", "input.mp4")?;
    
    // Generate experiments
    let experiments = skill.generate_experiments()?;
    
    // Run harness
    let harness = Harness::builder()
        .time_budget(std::time::Duration::from_secs(300))
        .build()?;
    
    let mut best_result = None;
    let mut best_score = 0.0;
    
    for exp in experiments {
        let result = harness.run(exp).await?;
        
        if result.metrics.val_bpb < best_score {
            best_score = result.metrics.val_bpb;
            best_result = Some(result);
        }
    }
    
    println!("🏆 Best configuration:");
    println!("{:#?}", best_result);
    
    Ok(())
}
```

### 3. Autonomous Overnight Run

```rust
use autocodeharness::{
    research::ResearchEngine,
    harness::Harness,
    codebase::CodebaseAnalyzer,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let analyzer = CodebaseAnalyzer::new(".")?;
    let mut engine = ResearchEngine::new(analyzer);
    let harness = Harness::builder()
        .time_budget(std::time::Duration::from_secs(300))
        .build()?;
    
    // Set baseline from first encoding
    let baseline = harness.run(
        engine.design_experiment(&Hypothesis {
            id: "baseline".to_string(),
            description: "H.264 CRF 23 medium preset".to_string(),
            rationale: "Standard YouTube settings".to_string(),
            expected_impact: 0.0,
            risk_level: RiskLevel::Low,
        })?
    ).await?;
    
    println!("📊 Baseline: VMAF {:.2}, Size {:.1}MB", 
        baseline.metrics.val_bpb, 
        baseline.metrics.peak_vram_mb / 1000.0
    );
    
    // Run autonomous search overnight
    loop {
        let hypothesis = engine.generate_hypothesis().await?;
        let experiment = engine.design_experiment(&hypothesis)?;
        let result = harness.run(experiment).await?;
        
        if result.metrics.improved_over(&baseline.metrics) {
            engine.commit(&hypothesis)?;
            println!("✓ Found better: VMAF {:.2}, Size {:.1}MB", 
                result.metrics.val_bpb,
                result.metrics.peak_vram_mb / 1000.0
            );
        } else {
            engine.revert()?;
        }
        
        // Run 50 experiments then stop
        if hypothesis.id.ends_with("50") {
            break;
        }
    }
    
    Ok(())
}
```

## Example Results

After running overnight, you might see:

```
📊 Experiment Results
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Baseline (H.264, CRF 23, medium):
  VMAF:     87.2
  Size:     45.3 MB
  Bitrate:  2,400 kbps
  Time:     1m 23s

Best Found (H.265, CRF 22, slow):
  VMAF:     91.5  ✨ +4.3 improvement!
  Size:     38.1 MB  🎯 15.9% smaller
  Bitrate:  2,020 kbps
  Time:     2m 47s

Trade-offs:
  ✓ Better quality (+4.3 VMAF)
  ✓ Smaller file (-7.2 MB)
  ✗ Slower encoding (+1m 24s)
  
Recommendation: Use H.265 CRF 22 slow for archival quality
                Use H.264 CRF 24 fast for quick uploads
```

## Metrics Explained

### VMAF (Video Multimethod Assessment Fusion)
- Industry-standard perceptual video quality metric
- Scale: 0-100 (higher is better)
- 95+ : Excellent (indistinguishable from source)
- 85-95: Very good (minimal artifacts)
- 75-85: Good (acceptable for streaming)
- <75  : Poor (visible compression artifacts)

### CRF (Constant Rate Factor)
- Quality vs file size knob
- Scale: 0-51 (lower is better quality)
- 18: Visually lossless
- 23: Default (YouTube 1080p)
- 28: Noticeable compression

## Requirements

- FFmpeg with libx264, libx265, libaom-av1
- VMAF model files (included in FFmpeg 4.2+)

```bash
# Install FFmpeg (macOS)
brew install ffmpeg

# Install FFmpeg (Ubuntu)
sudo apt install ffmpeg

# Verify VMAF support
ffmpeg -filters | grep vmaf
```

## Output

All results are logged to `experiments/video-optimization/results.tsv`:

```tsv
commit    vmaf_score  file_size_mb  encoding_time  status  description
a1b2c3d   87.2       45.3          83            keep    H.264 CRF 23 medium baseline
b2c3d4e   91.5       38.1          167           keep    H.265 CRF 22 slow (best quality)
c3d4e5f   85.1       35.2          58            keep    H.264 CRF 24 fast (best speed)
```

## Tips

1. **Start with baselines**: Test known-good settings first (H.264 CRF 23)
2. **GPU acceleration**: Use `-hwaccel` flags for faster encoding
3. **Two-pass encoding**: For better bitrate control in production
4. **Source quality**: Garbage in, garbage out - use high-quality source
5. **Validate subjectively**: VMAF is good but watch the output yourself

## Advanced: Custom Metrics

Edit `skill.toml` to add custom metrics:

```toml
[metrics.custom]
psnr_y = "higher_is_better"
ssim = "higher_is_better"
file_size_per_minute = "lower_is_better"
```

Then update your encoding script to output these metrics.

## Troubleshooting

**"VMAF not found"**: Update FFmpeg or install with VMAF support  
**"Too slow"**: Use GPU acceleration or faster presets  
**"Low scores"**: Check source quality and try lower CRF values  
**"OOM"**: Reduce resolution or use hardware encoding

## See Also

- [FFmpeg Encoding Guide](https://trac.ffmpeg.org/wiki/Encode/H.264)
- [VMAF Documentation](https://github.com/Netflix/vmaf)
- [Video Encoding Best Practices](https://gist.github.com/Vestride/278e13915894821e1d6f)
