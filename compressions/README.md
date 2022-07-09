<!-- Related to https://app.clickup.com/t/21gbxpc -->
<!-- https://blog.cloudflare.com/squeezing-the-firehose/ -->
<!-- Gzip: 0 arrives as uncompressed. -->
Goal is to compare speed/compression ratios when uploading/downloading from redpanda for:

By algorithm:

- gzip 1 - 12
- zstd 
- lz4
- snappy




|algorithm|compression_level|t_cumulative(s)|t_per_msg(s)|compressed_to|
|:---     |:---             |:---           |:---        |:---         |
|snappy   |0                |1920           |0.5         | 583 MB      |
|lz4      |1                |2651           |0.6         | 499 MB      |
|lz4      |2                |2692           |0.6         | 498 MB      |
|lz4      |5                |2710           |0.6         | 456 MB      |
|lz4      |7                |2710           |0.6         | 454 MB      |
|lz4      |9                |2705           |0.6         | 453 MB      |
|lz4      |12               |2694           |0.7         | 452 MB      |
|gzip     |1                |3740           |0.9         | 390 MB      |
|gzip     |2                |3780           |0.95        | 383 MB      |
|gzip     |3                |3776           |0.93        | 367 MB      |
|gzip     |5                |3764           |0.92        | 346 MB      |
|gzip     |7                |3756           |0.91        | 331 MB      |
|gzip     |12               |3747           |0.96        | 324 MB      |
|zstd     | -               |563            |0.37        | 267 MB      |







