# Data Import

- https://diesel.rs/
- https://www.kaggle.com/datasets/rounakbanik/pokemon

## How to Migration

### migrate
```bash
diesel migration run
```

### redo
```bash
# one
diesel migration redo

# all
diesel migration redo --all
```

### revert
```bash
# one
diesel migration revert

## all
diesel migration revert --all
```
