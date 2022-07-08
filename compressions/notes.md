<!-- Related to https://app.clickup.com/t/21gbxpc -->

Goal is to compare speed/compression ratios when uploading/downloading from redpanda for:

- zstd
- gzip
- lz4

1. Grab 10GB worth of data from the modern topic as json: dataset 1.
2. Shred the data into raw binary to simulate encoded data(actually not sure if there's any difference between the two): dataset 2.





