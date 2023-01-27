# IDS721 Spring 2023 Project 1 - Rust CLI Tool

The current [AWS CLI](https://github.com/aws/aws-cli/tree/v2) is written in Python. For this project I have built an AWS S3 CLI on Rust.

## Project Goals/Outcomes

* Develop my first Rust project
* Use Github Codespaces and Copilot
* Integrate the 'AWS SDK for Rust' into a Rust project

## Useage

1. Create an [AWS IAM User Policy for S3](https://docs.aws.amazon.com/AmazonS3/latest/userguide/security-iam-awsmanpol.html)

2. Configure your [~/.aws/credentials file](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html#cli-configure-files-where)

3. Run
```
cargo run
```

## Progress Log

**Week 1**
- [x] Create an [AWS IAM User Policy for S3](https://docs.aws.amazon.com/AmazonS3/latest/userguide/security-iam-awsmanpol.html)
- [x] Configure Github Codespaces with [AWS Toolkit Credential Profile](https://docs.aws.amazon.com/toolkit-for-vscode/latest/userguide/setup-credentials.html)
- [x] Initialise Rust project with [AWS SDK for Rust](https://github.com/awslabs/aws-sdk-rust)
- [x] Establish basic AWS client connection to list S3 buckets

**Week 2**
- [ ] Add clap command line parsing for arguments (bucket name, local file name)
- [ ] Bucket fxns: create, list
- [ ] Object fxns: list objects in bucket, upload to existing bucket, upload to new bucket
- [ ] Include: github release, containerization, benchmark, CI/CD


## References

* [AWS SDK for Rust](https://github.com/awslabs/aws-sdk-rust)
* [AWS Toolkit Credential Profile](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html#cli-configure-files-where)
* [AWS Credentials for VS Code](https://docs.aws.amazon.com/toolkit-for-vscode/latest/userguide/setup-credentials.html)
* [AWS IAM User Policy for S3](https://docs.aws.amazon.com/AmazonS3/latest/userguide/security-iam-awsmanpol.html)
* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)