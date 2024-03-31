# rust-process-csv
A program to process a specific csv file in Rust

# Requirements installs
Firstly and only you need to [Install Rust](https://www.rust-lang.org/tools/install).

# Before Running it
Before running it you need to put two CSV files in the root of the project, those are:
- **payments.csv** with all the payments extracted from grafana;
- **consents.csv** with all the consents extracted from grafana.

# How to run
A complex rust application is called a cargo, and it's the case of this application, for it you have to run the following command:

``cargo run {filename.csv}``

Example (if the file is in the root of the project):

``cargo run test.csv``