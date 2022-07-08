<!-- Related to https://app.clickup.com/t/21gbxpc -->

Goal is to compare speed/compression ratios when uploading/downloading from redpanda for:

By algorithm:

- gzip
- zstd
- lz4
- snappy

By modality:

- from file to topic
- from topic to topic
- from topic to file


1. Grab 10GB worth of data from the modern topic as json: dataset 1.
2. Shred the data into raw binary to simulate encoded data(actually not sure if there's any difference between the two): dataset 2.

Funnel about 10 gigs of data from modern blocks to a separate topic for safety.






