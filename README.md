# Pokemon Open Data

The Complete Pokemon Dataset
- https://www.kaggle.com/datasets/rounakbanik/pokemon

## Docs
- https://diesel.rs/guides/getting-started

## MySQL で LOAD DATA する際の設定確認

```sql
-- ファイル読み込みが許可されているか
SELECT @@local_infile;

-- CSVの設置場所を確認する
SELECT @@global.secure_file_priv;
```
