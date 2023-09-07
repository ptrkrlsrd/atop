# ATOP
## Simple TOP Like Process Monitoring Tool with averages

## Overview

This is a simple process monitoring tool written in Rust. It uses the `sysinfo` library to gather information about running processes and their resource usage. The tool displays the processes sorted by their average CPU usage, filtering out processes below a certain CPU usage threshold.

## Features

- Displays process name, average CPU usage, current CPU usage, average memory usage, and current memory usage.
- Updates the display every second.
- Filters out processes with average CPU usage below 0.1%.


## How to install

1. Make sure you have Rust and Cargo installed.
2. Run

    ```bash
    cargo install atop
    ```

## License
MIT
