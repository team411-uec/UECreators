# UECreator
UECreator

## プロジェクト構成
- backend: バックエンド
- frontend: フロントエンド
- shared: 共通の型定義やユーティリティ関数
各ディレクトリの詳細は`/doc/directory.txt`を参照してください

## 開発環境の構築
### パッケージのインストール
- frontendディレクトリで`npm install`
### .envファイルの設定
- backend/.envのPORTに適当な値を設定
- frontend/.envのVITE_API_URLに`http://localhost:{上で設定したポート番号}`を設定

## 開発サーバーの起動
### フロントエンド
- frontendディレクトリで`npm run dev`
### バックエンド
- backendディレクトリで`cargo run`