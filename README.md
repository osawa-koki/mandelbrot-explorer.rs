# mandelbrot-explorer.rs

😴😴😴 Rustでマンデルブロ集合を描画しよう！  

![成果物](./docs/img/fruit.gif)  

## 実行方法

最初に、`.env.example`を`.env`にリネームしてください。  
また、各種パラメータを設定してください。  

| Name | Description |
| ---- | ---- |
| FROM | 開始フレーム |
| UPTO | 終了フレーム |
| ASPECT_RATIO | アスペクト比 |
| SHRINK_RATIO | 縮小率 |
| FILESIZE_HEIGHT | 画像の高さ |
| START_X | 描画する範囲の左上のx座標 |
| START_Y | 描画する範囲の左上のy座標 |
| DEFAULT_WIDTH | 描画する範囲の幅 |
| DEFAULT_HEIGHT | 描画する範囲の高さ |

```shell
docker build -t mandelbrot-explorer-rs .
docker run -it --name mandelbrot-explorer-rs mandelbrot-explorer-rs
docker cp mandelbrot-explorer-rs:/work/fruit.mp4 fruit.mp4
```

`fruit.mp4`が生成されるので、それを再生するとマンデルブロ集合が描画されているのがわかります！  

---

Dockerではなく、ローカルで実行する場合は以下のようにしてください。  

```shell
cargo build --release
cargo run --release
```

`./dist/`ディレクトリにマンデルブロ集合の画像が生成されます。  
これをもとに、`ffmpeg`などで動画を生成してください。  

コマンドは以下のようになります。  

```shell
ffmpeg -r 30 -i ./dist/%08d.png -vcodec libx264 -pix_fmt yuv420p -r 60 ./fruit.mp4
```
