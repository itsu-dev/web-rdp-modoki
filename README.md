# web-rdp-modoki
2024年の正月に作った、Rustで実装されたRDPクライアントもどきです。Webを通して他のPCを操作することができます。  
画面の配信はMJPEG over HTTP、各遠隔操作はサボったのでAPIエンドポイント直叩きで実装しています（余力があればWebRTCによる実装に変更したい）。  
なおMacのみ動作確認を行っています。

<div><video controls src="https://video.twimg.com/ext_tw_video/1743302162434924544/pu/vid/avc1/1280x720/n8cYqQMCqVVdTb8-.mp4?tag=12" muted="true"></video></div>

## 使用方法
### ホスト側
1. このリポジトリをクローンする
```sh
git clone git@github.com:itsu-dev/web-rdp-modoki.git
```
2. ビルドして実行
```sh
cargo run --release
```

### クライアント側
任意のWebブラウザを開き、ホスト側PCのIPアドレス（ポート80番）にアクセスします。例は次のとおりです。
```
http://192.168.1.100:8080
```