
### 一个网页上wasm的例子，可以画直线，矩形，圆形，可以拖到控制点进行移动和缩放
#### 项目结构使用 wasm-pack-template模板
  cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name wasm-draw
#### 使用到的crates:
+ wasm-bindgen
+ web_sys
+ js_sys

