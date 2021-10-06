# EC2 and STS Describer

This was my first app using Rust. Learning basic things such as `match` 

I wrote a Python script that does the exact thing with the `boto3` module. I timed the execution and here are my results:

```
$ time python3 app.py

Instances in AWS Account ID 659892707512
        - i-0a30ac4e196e13320
        - i-0ba71e8950cc72ba1
        - i-041bb4a6f937c684d
        - i-0c14e224dc2e4b2f1
        - i-0002b03d5442655fe
        - i-00b7b0d0b2a436711
        - i-0f30f46c105cf5de5
        - i-040cb4273cf6b45ba
        - i-02da56657b24ba33d
        - i-0664f7ca15eb0e54e
        - i-005073bc2f088df72
        - i-0483613735a3bf27c
        - i-0dc01d0ccb6b66b69
        - i-07e79a9c1a54d2065
        - i-08719758514f336a2
        - i-033aafef0d89ff304


python3 app.py  0.30s user 0.11s system 24% cpu 1.657 total
```

And the timing results from the Rust build
```
time ../target/release/ec2-reporter-rs 

Instances in AWS Account ID 659892707512
        - i-0a30ac4e196e13320
        - i-0ba71e8950cc72ba1
        - i-041bb4a6f937c684d
        - i-0c14e224dc2e4b2f1
        - i-0002b03d5442655fe
        - i-00b7b0d0b2a436711
        - i-0f30f46c105cf5de5
        - i-040cb4273cf6b45ba
        - i-02da56657b24ba33d
        - i-0664f7ca15eb0e54e
        - i-005073bc2f088df72
        - i-0483613735a3bf27c
        - i-0dc01d0ccb6b66b69
        - i-07e79a9c1a54d2065
        - i-08719758514f336a2
        - i-033aafef0d89ff304


../target/release/ec2-reporter-rs  0.03s user 0.02s system 4% cpu 1.059 total
```

I know I'm late to the Rust party, and excuse me sounding naive, but holy speed! I'm sold on this language.


## Prerequisites

- AWS credentials set up in your environment variables
- Rust to compile

## Technologies used

- tokio
- rusoto
