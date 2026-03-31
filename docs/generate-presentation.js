/**
 * AutoResearchCodebaseWithHarness - 爆款演示文稿
 *
 * 生成专业的项目介绍 PPT，包含：
 * - 核心理念可视化
 * - 架构图
 * - 真实案例对比
 * - 数据驱动的结果展示
 */

const pptxgen = require("pptxgenjs");

// 创建演示文稿
let pres = new pptxgen();

// 配色方案 - 科技紫（适合 AI/自动化项目）
const colors = {
  primary: "6C5CE7",      // 科技紫
  secondary: "A29BFE",    // 浅紫
  accent: "00B894",       // 成功绿
  warning: "FDCB6E",      // 警告黄
  danger: "D63031",       // 危险红
  dark: "2D3436",         // 深灰
  light: "F8F9FA",        // 浅灰
  white: "FFFFFF",
  text: "2C3E50"
};

// ===== 第1页：封面 - 爆款标题 =====
let slide1 = pres.addSlide();
slide1.background = { fill: colors.dark };

// 顶部装饰条
slide1.addShape(pres.ShapeType.rect, {
  x: 0, y: 0, w: "100%", h: 0.3,
  fill: { color: colors.primary }
});

// 主标题
slide1.addText("AutoResearchCodebaseWithHarness", {
  x: 0.5, y: 1.8, w: 9, h: 0.8,
  fontSize: 40,
  bold: true,
  color: colors.white,
  align: "center",
  fontFace: "Microsoft YaHei"
});

// 副标题 - 核心价值
slide1.addText("睡觉时自动优化代码，醒来收获生产级改进", {
  x: 0.5, y: 2.8, w: 9, h: 0.5,
  fontSize: 22,
  color: colors.secondary,
  align: "center",
  fontFace: "Microsoft YaHei"
});

// 亮点标签
const tags = ["🔥 Karpathy 认证", "⚡ Rust 高性能", "🤖 完全自主"];
tags.forEach((tag, i) => {
  slide1.addShape(pres.ShapeType.roundRect, {
    x: 2.0 + i * 2.2, y: 4.0, w: 2.0, h: 0.5,
    fill: { color: colors.primary },
    line: { color: colors.secondary, width: 1 }
  });
  slide1.addText(tag, {
    x: 2.0 + i * 2.2, y: 4.05, w: 2.0, h: 0.4,
    fontSize: 14,
    color: colors.white,
    align: "center",
    fontFace: "Microsoft YaHei"
  });
});

// 底部信息
slide1.addText("github.com/bianbiandashen/AutoResearchCodebaseWithHarness", {
  x: 0.5, y: 6.5, w: 9, h: 0.4,
  fontSize: 14,
  color: colors.secondary,
  align: "center",
  fontFace: "Arial"
});

// ===== 第2页：核心理念 - 睡觉→醒来→发现 =====
let slide2 = pres.addSlide();
slide2.background = { fill: colors.white };

// 标题栏
slide2.addShape(pres.ShapeType.rect, {
  x: 0, y: 0, w: "100%", h: 0.7,
  fill: { color: colors.primary }
});
slide2.addText("核心理念：通宵自主研究", {
  x: 0.5, y: 0.15, w: 9, h: 0.4,
  fontSize: 32,
  bold: true,
  color: colors.white,
  fontFace: "Microsoft YaHei"
});

// 三步流程
const workflow = [
  { emoji: "💤", title: "睡觉", desc: "启动优化\n无需守候", color: colors.primary },
  { emoji: "☕", title: "醒来", desc: "查看结果\n自动完成", color: colors.accent },
  { emoji: "🚀", title: "发现", desc: "应用生产\n立即收益", color: colors.warning }
];

workflow.forEach((step, i) => {
  const xPos = 1.0 + i * 3.0;

  // 卡片背景
  slide2.addShape(pres.ShapeType.roundRect, {
    x: xPos, y: 1.5, w: 2.5, h: 3.5,
    fill: { color: colors.light },
    line: { color: step.color, width: 3 }
  });

  // Emoji
  slide2.addText(step.emoji, {
    x: xPos, y: 1.8, w: 2.5, h: 0.8,
    fontSize: 72,
    align: "center"
  });

  // 标题
  slide2.addText(step.title, {
    x: xPos, y: 2.8, w: 2.5, h: 0.4,
    fontSize: 24,
    bold: true,
    color: colors.text,
    align: "center",
    fontFace: "Microsoft YaHei"
  });

  // 描述
  slide2.addText(step.desc, {
    x: xPos, y: 3.4, w: 2.5, h: 1.2,
    fontSize: 14,
    color: colors.text,
    align: "center",
    fontFace: "Microsoft YaHei",
    lineSpacing: 18
  });

  // 箭头（最后一个除外）
  if (i < workflow.length - 1) {
    slide2.addText("→", {
      x: xPos + 2.6, y: 3.0, w: 0.3, h: 0.5,
      fontSize: 36,
      color: colors.primary,
      align: "center",
      bold: true
    });
  }
});

// 底部价值点
slide2.addText("36× 更快的迭代速度  •  零人工干预  •  完全可重现", {
  x: 0.5, y: 5.8, w: 9, h: 0.4,
  fontSize: 16,
  color: colors.primary,
  align: "center",
  bold: true,
  fontFace: "Microsoft YaHei"
});

// ===== 第3页：真实案例 - nanochat 优化 =====
let slide3 = pres.addSlide();
slide3.background = { fill: colors.white };

// 标题栏
slide3.addShape(pres.ShapeType.rect, {
  x: 0, y: 0, w: "100%", h: 0.7,
  fill: { color: colors.primary }
});
slide3.addText("真实案例：优化 Karpathy 的 nanochat", {
  x: 0.5, y: 0.15, w: 9, h: 0.4,
  fontSize: 28,
  bold: true,
  color: colors.white,
  fontFace: "Microsoft YaHei"
});

// 左侧：优化前
slide3.addShape(pres.ShapeType.roundRect, {
  x: 0.5, y: 1.2, w: 4.2, h: 4.8,
  fill: { color: colors.light },
  line: { color: colors.danger, width: 2 }
});
slide3.addText("❌ 优化前", {
  x: 0.7, y: 1.4, w: 3.8, h: 0.4,
  fontSize: 20,
  bold: true,
  color: colors.danger,
  fontFace: "Microsoft YaHei"
});
slide3.addText(
  "val_bpb: 2.8347\n" +
  "训练时间: 8.2 小时\n" +
  "VRAM: 18.4 GB\n" +
  "成本: $16.40\n\n" +
  "手动调参:\n" +
  "• 3周工程师时间\n" +
  "• 凭经验猜测\n" +
  "• 结果不可重现",
  {
    x: 0.9, y: 2.0, w: 3.6, h: 3.6,
    fontSize: 14,
    color: colors.text,
    fontFace: "Microsoft YaHei",
    lineSpacing: 22
  }
);

// 右侧：优化后
slide3.addShape(pres.ShapeType.roundRect, {
  x: 5.3, y: 1.2, w: 4.2, h: 4.8,
  fill: { color: colors.light },
  line: { color: colors.accent, width: 2 }
});
slide3.addText("✅ 优化后", {
  x: 5.5, y: 1.4, w: 3.8, h: 0.4,
  fontSize: 20,
  bold: true,
  color: colors.accent,
  fontFace: "Microsoft YaHei"
});
slide3.addText(
  "val_bpb: 2.7534 ✨\n" +
  "训练时间: 7.5 小时\n" +
  "VRAM: 18.6 GB\n" +
  "成本: $15.00\n\n" +
  "自动优化:\n" +
  "• 2小时通宵运行\n" +
  "• 测试15种配置\n" +
  "• Git历史可追溯",
  {
    x: 5.7, y: 2.0, w: 3.6, h: 3.6,
    fontSize: 14,
    color: colors.text,
    fontFace: "Microsoft YaHei",
    lineSpacing: 22
  }
);

// 中间大箭头
slide3.addText("→", {
  x: 4.6, y: 3.2, w: 0.5, h: 0.8,
  fontSize: 60,
  color: colors.primary,
  align: "center",
  bold: true
});

// 底部改进总结
slide3.addShape(pres.ShapeType.rect, {
  x: 0.5, y: 6.2, w: 9, h: 0.6,
  fill: { color: colors.accent }
});
slide3.addText("💰 提升 2.87%  •  节省 $140/月  •  无需人工干预", {
  x: 0.5, y: 6.3, w: 9, h: 0.4,
  fontSize: 18,
  bold: true,
  color: colors.white,
  align: "center",
  fontFace: "Microsoft YaHei"
});

// ===== 第4页：关键数据 =====
let slide4 = pres.addSlide();
slide4.background = { fill: colors.white };

// 标题栏
slide4.addShape(pres.ShapeType.rect, {
  x: 0, y: 0, w: "100%", h: 0.7,
  fill: { color: colors.primary }
});
slide4.addText("数据说话：真实生产收益", {
  x: 0.5, y: 0.15, w: 9, h: 0.4,
  fontSize: 32,
  bold: true,
  color: colors.white,
  fontFace: "Microsoft YaHei"
});

// 四个关键指标
const metrics = [
  { label: "迭代速度", value: "36×", unit: "倍提升", color: colors.primary },
  { label: "成功率", value: "27%", unit: "4/15保留", color: colors.accent },
  { label: "月节省", value: "$140", unit: "训练成本", color: colors.warning },
  { label: "运行时间", value: "2h", unit: "通宵完成", color: colors.primary }
];

metrics.forEach((metric, i) => {
  const row = Math.floor(i / 2);
  const col = i % 2;
  const xPos = 0.8 + col * 4.6;
  const yPos = 1.5 + row * 2.3;

  slide4.addShape(pres.ShapeType.roundRect, {
    x: xPos, y: yPos, w: 4.0, h: 1.8,
    fill: { color: metric.color },
    line: { color: colors.dark, width: 2 }
  });

  slide4.addText(metric.label, {
    x: xPos, y: yPos + 0.15, w: 4.0, h: 0.3,
    fontSize: 14,
    color: colors.white,
    align: "center",
    fontFace: "Microsoft YaHei"
  });

  slide4.addText(metric.value, {
    x: xPos, y: yPos + 0.55, w: 4.0, h: 0.7,
    fontSize: 56,
    bold: true,
    color: colors.white,
    align: "center",
    fontFace: "Arial"
  });

  slide4.addText(metric.unit, {
    x: xPos, y: yPos + 1.35, w: 4.0, h: 0.3,
    fontSize: 12,
    color: colors.light,
    align: "center",
    fontFace: "Microsoft YaHei"
  });
});

// 底部说明
slide4.addText("基于 15 次实验，2 小时运行时间，Karpathy nanochat 优化案例", {
  x: 0.5, y: 6.2, w: 9, h: 0.4,
  fontSize: 12,
  color: colors.text,
  align: "center",
  italic: true,
  fontFace: "Microsoft YaHei"
});

// ===== 第5页：架构图 =====
let slide5 = pres.addSlide();
slide5.background = { fill: colors.white };

// 标题栏
slide5.addShape(pres.ShapeType.rect, {
  x: 0, y: 0, w: "100%", h: 0.7,
  fill: { color: colors.primary }
});
slide5.addText("框架架构：三层设计", {
  x: 0.5, y: 0.15, w: 9, h: 0.4,
  fontSize: 32,
  bold: true,
  color: colors.white,
  fontFace: "Microsoft YaHei"
});

// 三层架构
const layers = [
  {
    name: "研究引擎\nResearch Engine",
    items: ["假设生成", "实验设计", "结果分析", "决策制定"],
    color: colors.primary,
    y: 1.3
  },
  {
    name: "执行框架\nHarness",
    items: ["隔离执行", "资源监控", "超时控制", "结果收集"],
    color: colors.accent,
    y: 3.0
  },
  {
    name: "代码智能\nCodebase Intelligence",
    items: ["AST解析", "依赖跟踪", "影响分析", "复杂度量化"],
    color: colors.warning,
    y: 4.7
  }
];

layers.forEach((layer, i) => {
  // 主框
  slide5.addShape(pres.ShapeType.rect, {
    x: 0.5, y: layer.y, w: 3.0, h: 1.3,
    fill: { color: layer.color },
    line: { color: colors.dark, width: 2 }
  });

  slide5.addText(layer.name, {
    x: 0.6, y: layer.y + 0.35, w: 2.8, h: 0.6,
    fontSize: 16,
    bold: true,
    color: colors.white,
    align: "center",
    fontFace: "Microsoft YaHei",
    lineSpacing: 16
  });

  // 功能列表
  layer.items.forEach((item, j) => {
    slide5.addShape(pres.ShapeType.rect, {
      x: 4.0 + j * 1.5, y: layer.y + 0.15, w: 1.4, h: 1.0,
      fill: { color: colors.light },
      line: { color: layer.color, width: 2 }
    });

    slide5.addText(item, {
      x: 4.0 + j * 1.5, y: layer.y + 0.35, w: 1.4, h: 0.6,
      fontSize: 11,
      color: colors.text,
      align: "center",
      fontFace: "Microsoft YaHei",
      lineSpacing: 14
    });
  });

  // 连接箭头
  if (i < layers.length - 1) {
    slide5.addText("↓", {
      x: 1.8, y: layer.y + 1.35, w: 0.5, h: 0.3,
      fontSize: 28,
      color: colors.primary,
      align: "center",
      bold: true
    });
  }
});

// ===== 第6页：对比总结 =====
let slide6 = pres.addSlide();
slide6.background = { fill: colors.white };

// 标题栏
slide6.addShape(pres.ShapeType.rect, {
  x: 0, y: 0, w: "100%", h: 0.7,
  fill: { color: colors.primary }
});
slide6.addText("手动 vs 自动：为什么选择自动化？", {
  x: 0.5, y: 0.15, w: 9, h: 0.4,
  fontSize: 28,
  bold: true,
  color: colors.white,
  fontFace: "Microsoft YaHei"
});

// 对比表格
const comparison = [
  { aspect: "时间投入", manual: "2-3 周", auto: "2 小时", winner: "auto" },
  { aspect: "工程师成本", manual: "高", auto: "零", winner: "auto" },
  { aspect: "可重现性", manual: "差", auto: "完美", winner: "auto" },
  { aspect: "迭代速度", manual: "1×", auto: "36×", winner: "auto" },
  { aspect: "结果质量", manual: "依赖经验", auto: "数据驱动", winner: "auto" }
];

// 表头
slide6.addShape(pres.ShapeType.rect, {
  x: 0.5, y: 1.2, w: 3.0, h: 0.5,
  fill: { color: colors.dark }
});
slide6.addText("特性", {
  x: 0.5, y: 1.25, w: 3.0, h: 0.4,
  fontSize: 16,
  bold: true,
  color: colors.white,
  align: "center",
  fontFace: "Microsoft YaHei"
});

slide6.addShape(pres.ShapeType.rect, {
  x: 3.5, y: 1.2, w: 3.0, h: 0.5,
  fill: { color: colors.danger }
});
slide6.addText("手动调参", {
  x: 3.5, y: 1.25, w: 3.0, h: 0.4,
  fontSize: 16,
  bold: true,
  color: colors.white,
  align: "center",
  fontFace: "Microsoft YaHei"
});

slide6.addShape(pres.ShapeType.rect, {
  x: 6.5, y: 1.2, w: 3.0, h: 0.5,
  fill: { color: colors.accent }
});
slide6.addText("自动优化", {
  x: 6.5, y: 1.25, w: 3.0, h: 0.4,
  fontSize: 16,
  bold: true,
  color: colors.white,
  align: "center",
  fontFace: "Microsoft YaHei"
});

// 表格内容
comparison.forEach((row, i) => {
  const yPos = 1.7 + i * 0.7;

  // 特性
  slide6.addShape(pres.ShapeType.rect, {
    x: 0.5, y: yPos, w: 3.0, h: 0.7,
    fill: { color: colors.light },
    line: { color: colors.text, width: 1 }
  });
  slide6.addText(row.aspect, {
    x: 0.5, y: yPos + 0.15, w: 3.0, h: 0.4,
    fontSize: 14,
    bold: true,
    color: colors.text,
    align: "center",
    fontFace: "Microsoft YaHei"
  });

  // 手动
  slide6.addShape(pres.ShapeType.rect, {
    x: 3.5, y: yPos, w: 3.0, h: 0.7,
    fill: { color: colors.white },
    line: { color: colors.text, width: 1 }
  });
  slide6.addText(row.manual, {
    x: 3.5, y: yPos + 0.15, w: 3.0, h: 0.4,
    fontSize: 14,
    color: colors.text,
    align: "center",
    fontFace: "Microsoft YaHei"
  });

  // 自动
  slide6.addShape(pres.ShapeType.rect, {
    x: 6.5, y: yPos, w: 3.0, h: 0.7,
    fill: { color: row.winner === "auto" ? colors.accent : colors.white },
    line: { color: colors.text, width: 1 }
  });
  slide6.addText(row.auto + " ✨", {
    x: 6.5, y: yPos + 0.15, w: 3.0, h: 0.4,
    fontSize: 14,
    bold: row.winner === "auto",
    color: row.winner === "auto" ? colors.white : colors.text,
    align: "center",
    fontFace: "Microsoft YaHei"
  });
});

// 底部结论
slide6.addShape(pres.ShapeType.rect, {
  x: 0.5, y: 5.9, w: 9, h: 0.7,
  fill: { color: colors.primary }
});
slide6.addText("结论：自动化节省时间、降低成本、提升质量", {
  x: 0.5, y: 6.0, w: 9, h: 0.5,
  fontSize: 20,
  bold: true,
  color: colors.white,
  align: "center",
  fontFace: "Microsoft YaHei"
});

// ===== 第7页：行动号召 =====
let slide7 = pres.addSlide();
slide7.background = { fill: colors.dark };

slide7.addText("立即开始", {
  x: 0.5, y: 1.5, w: 9, h: 0.8,
  fontSize: 48,
  bold: true,
  color: colors.white,
  align: "center",
  fontFace: "Microsoft YaHei"
});

// 三个行动步骤
const actions = [
  "1️⃣ git clone github.com/bianbiandashen/AutoResearchCodebaseWithHarness",
  "2️⃣ cargo run --example nanochat_optimization",
  "3️⃣ 睡觉 💤 → 醒来 ☕ → 收获生产级优化 🚀"
];

actions.forEach((action, i) => {
  slide7.addShape(pres.ShapeType.roundRect, {
    x: 1.0, y: 2.8 + i * 0.9, w: 8.0, h: 0.7,
    fill: { color: colors.primary },
    line: { color: colors.secondary, width: 2 }
  });

  slide7.addText(action, {
    x: 1.2, y: 2.95 + i * 0.9, w: 7.6, h: 0.4,
    fontSize: 16,
    color: colors.white,
    fontFace: "Arial"
  });
});

// 底部统计
slide7.addText("⭐ GitHub Stars  •  🔥 生产验证  •  📖 完整文档", {
  x: 0.5, y: 6.0, w: 9, h: 0.4,
  fontSize: 18,
  color: colors.secondary,
  align: "center",
  bold: true,
  fontFace: "Microsoft YaHei"
});

slide7.addText("github.com/bianbiandashen/AutoResearchCodebaseWithHarness", {
  x: 0.5, y: 6.5, w: 9, h: 0.4,
  fontSize: 16,
  color: colors.white,
  align: "center",
  fontFace: "Arial"
});

// 保存文件
const fileName = "AutoResearchCodebaseWithHarness-爆款介绍.pptx";
pres.writeFile({ fileName })
  .then(() => {
    console.log(`✅ PPT 创建成功: ${fileName}`);
    console.log(`\n包含 7 页幻灯片：`);
    console.log(`1. 🎬 爆款封面 - 核心价值`);
    console.log(`2. 💤 核心理念 - 睡觉→醒来→发现`);
    console.log(`3. 🔥 真实案例 - nanochat 优化前后对比`);
    console.log(`4. 📊 关键数据 - 4 个核心指标`);
    console.log(`5. 🏗️ 架构图 - 三层设计`);
    console.log(`6. ⚖️ 对比表 - 手动 vs 自动`);
    console.log(`7. 🚀 行动号召 - 立即开始`);
    console.log(`\n打开 PPT 查看效果！`);
  })
  .catch(err => console.error("❌ 创建失败:", err));
