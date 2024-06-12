# ベースイメージの設定
FROM rust:latest

# 作業ディレクトリの設定
WORKDIR /app

# 必要なファイルをコピー
COPY . .

# 依存関係のインストール
RUN cargo build --release

# アプリケーションの実行
CMD ["./target/release/backend"]
