# IDS721 Spring 2023 Project 1 - Rust CLI Tool

A Rust based CLI tool that allows the user to upload a local file to an AWS S3 bucket directly from the command line.

## Project Goals/Outcomes

* Develop my first Rust project
* Use Github Codespaces and Copilot
* Integrate the 'AWS SDK for Rust' into a Rust project

## Useage

```
cargo run
```

## Progress Log

**Week 1**
- [x] Create an [AWS IAM User Policy for S3](https://docs.aws.amazon.com/AmazonS3/latest/userguide/security-iam-awsmanpol.html)
- [x] Configure Github Codespaces with [AWS Toolkit Credential Profile](https://docs.aws.amazon.com/toolkit-for-vscode/latest/userguide/setup-credentials.html)
- [x] Initialise Rust project with [AWS SDK for Rust](https://github.com/awslabs/aws-sdk-rust)
- [x] Establish basic AWS client connection to list S3 buckets
- [ ] Add clap command line parsing for arguments (bucket name, local file name)
- [ ] Add flag to create bucket if DNE
- [ ] Include: github release, containerization, benchmark, CI/CD


## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)