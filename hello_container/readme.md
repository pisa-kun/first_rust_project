# RustアプリのコンテナイメージをLambdaで動かす

## Lambdaアプリの実装

プロジェクト作成
> cargo new hello_container --lib

--libをつけるとライブラリ作成モード(defaultはbinでmain.rs作成)

依存クレート、実行ファイルの指定
```toml
[package]
name = "hello_container"
version = "1.0.0"
authors = ["pisakun"]
edition = "2024"

# 実行ファイルの配置場所を変更しているため明示的に指定する
[[bin]]
name = "hello_container"
path = "src/bin/hello_container.rs"

[dependencies]
lambda_runtime = "0.2.1"
serde = { version = "1", features = ["derive"] }
```

コンパイル可能かの確認
> cargo check

## Lambdaコンテナ用のイメージ作成
イメージのビルド
> docker image build -t "hello-container-build" -f Dockerfile.build .

コンテナを立ち上げてビルド
> docker container run --rm -v $PWD:/code -v $HOME/.cargo/registry:/root/.cargo/registry -v $HOME/.cargo/git:/root/.cargo/git hello-container-build

ランタイム用のイメージをビルド
> docker image build -t "hello-container" -f Dockerfile .

※-f がない場合はデフォルトでDockerfileを指定するのでなくてもよい

## AWS環境にデプロイ

> aws ecr create-repository --repository-name hello-container

ログイン
>  aws ecr get-login-password --region ap-northeast-1 | docker login --username AWS --password-stdin xxxxxxxxxxxx.dkr.ecr.ap-northeast-1.amazonaws.com

イメージにタグ付け
> docker image tag hello-container:latest xxxxxxxxxxxx.dkr.ecr.ap-northeast-1.amazonaws.com/hello-container:latest

ECRにプッシュ
> docker image push xxxxxxxxxxxx.dkr.ecr.ap-northeast-1.amazonaws.com/hello-container:latest

digestの取得
> aws ecr list-images --repository-name hello-container --out text --query 'imageIds[?imageTag==`latest`].imageDigest'  

### lambdaの作成

> aws lambda create-function --function-name hello-rust-container --package-type Image --code ImageUri=<AccountID>.dkr.ecr.ap-northeast-1.amazonaws.com/hello-container@<digest> --role arn

lambdaの実行
> aws lambda invoke --function-name hello-rust-container --cli-binary-format raw-in-base64-out --payload '{ \"name\": \"Rust container Morichan\" }' outfile

出力結果
> cat outfile

> {"message":"Hello, Rust container Morichan!"}

### 参考

https://dev.classmethod.jp/articles/rust-app-container-on-lambda-function/

lambdaをコンテナから作成するときのCLIコマンド
https://aws.amazon.com/jp/builders-flash/202103/new-lambda-container-development/?awsf.filter-name=*all

## next step
AWS CDKでデプロイ
https://dev.classmethod.jp/articles/rust-lambda-cdk/