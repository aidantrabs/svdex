# svdex

an interactive cli tool for compressing images using truncated singular value decomposition.

## demo

![demo](assets/demo.gif)

## usage

### compress a single image

```bash
cargo run -- compress assets/images/picture.jpg -k 50
```

- `-k` - number of singular values to keep (default: 50)
- `-o` - custom output path (default: `output/compressed/compressed_k{k}.png`)

### run experiments at multiple ranks

```bash
cargo run -- experiment assets/images/picture.jpg
```

- `-r` - comma-separated ranks to test (default: 1,2,5,10,20,50,100,150,200)
- generates compressed images for each rank and a singular value decay plot at `experiments/sv_decay.png`

### inspect image info and singular values

```bash
cargo run -- info assets/images/picture.jpg
```

## project structure

```
src/
  main.rs         - entry point, subcommand dispatch
  cli.rs          - clap cli definitions
  image_io.rs     - image to matrix conversion
  matrix.rs       - matrix utility helpers
  svd.rs          - svd computation and low-rank approximation
  compression.rs  - per-channel compression pipeline
  metrics.rs      - mse, psnr, compression ratio
  experiment.rs   - multi-rank experiments and visualization
```

## dependencies

- `image` - image loading and saving
- `ndarray` + `ndarray-linalg` - matrix operations and svd
- `clap` - cli parsing
- `plotters` - singular value decay plots
- `anyhow` - error handling
