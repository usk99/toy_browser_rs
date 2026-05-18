# toy_browser_rs

Rust で自作した教育用ミニマルブラウザ。HTML/CSS のパースからレイアウト・描画まで、ブラウザのレンダリングパイプラインを一から実装している。

## 機能

- HTML サブセットのパース（タグ・テキストノード・コメント）
- CSS サブセットのパース（タグ/ID/クラスセレクタ、`background-color` / `height` / `padding` / `margin` 等）
- スタイル解決（DOM × CSS → StyledNode）
- ブロックレイアウト（`display: block`、padding / margin 対応）
- 描画（背景色の矩形 + テキスト、アルファブレンディングによるアンチエイリアス）
- ウィンドウ表示・リサイズ対応

## 使い方

```bash
# デフォルト（test.html / test.css を使用）
cargo run

# ファイルを指定
cargo run -- path/to/file.html path/to/file.css
```

WSL2 環境では WSLg（Windows 11 標準）があればそのまま動きます。X11 強制のため Wayland は自動で無効化されます。

```bash
# テスト
cargo test
```

## 依存クレート

| クレート      | 用途                         |
| ------------- | ---------------------------- |
| `winit`       | ウィンドウ管理・イベントループ |
| `softbuffer`  | ピクセルバッファ転送          |
| `tiny-skia`   | 矩形の 2D 描画               |
| `fontdue`     | フォントラスタライズ          |

## ライセンス

同梱フォント `assets/fonts/Ubuntu-R.ttf` は [Ubuntu Font Licence 1.0](https://ubuntu.com/legal/font-licence) のもとで配布されています。
