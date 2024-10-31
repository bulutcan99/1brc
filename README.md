## Overview

This project aims to test the capabilities of Rust in handling large-scale data processing challenges. The primary objective is to efficiently process **one billion rows** of data, evaluating Rust's performance, scalability, and efficiency in this context.

## System Specifications

- **Device**: MacBook Pro M2 Pro
- **CPU**: 10-core CPU
- **RAM**: 16 GB
- **Storage**: 512 GB SSD
- **GPU**: 16-core GPU

## Performance Metrics

The time taken to process **1 billion rows** of data:

- **First Processing Time**: **129.492 seconds** -> Brute force
- Second Processing Time: 185.944 seconds -> Parallel Processing
- Third Processing Time: 105.231 seconds -> Bytes instead of String
- Fourth Processing Time: 99.492 seconds -> Custom float type
- Fifth Processing Time: 90.205 seconds -> Map key to u64
- Sixth Processing Time: 88.782 seconds -> Vec initialize with capacity
- Seventh Processing Time: 27.383 seconds -> Rayon prallel processing
