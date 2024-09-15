# Pokemon Open Data

The Complete Pokemon Dataset
- https://www.kaggle.com/datasets/rounakbanik/pokemon

## api
⚙ Rust 1.81

Rest API の提供

## database
🐬 MySQL 8.4

## import
⚙ Rust 1.81

オープンデータの取込, migration の管理用
- https://diesel.rs/guides/getting-started

※ schema は使用しない

## MySQL で LOAD DATA する際の設定確認

```sql
-- ファイル読み込みが許可されているか
SELECT @@local_infile;

-- CSVの設置場所を確認する
SELECT @@global.secure_file_priv;
```
