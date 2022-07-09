<!-- Related to https://app.clickup.com/t/21gbxpc -->
<!-- https://blog.cloudflare.com/squeezing-the-firehose/ -->
<!-- Gzip: 0 arrives as uncompressed. -->
Goal is to compare speed/compression ratios when uploading/downloading from redpanda for:

By algorithm:

- gzip 1 - 12
- zstd 
- lz4
- snappy

By modality?:

- from file to topic
- from topic to topic
- from topic to file


1. Grab 10GB worth of data from the modern topic as json: dataset 1.

Funnel about 10 gigs of data from modern blocks to a separate topic for safety.






