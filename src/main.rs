/// Main entry point of the application.
///
/// This module manages the application start-up, by calling the `cli::start` which
/// functions as the command-line interface of the application.
/// If an error occurs during the execution, the error is printed to the standard output.
mod cli;

/// This module provides utility functions for operations related to file system,
/// such as loading the contents of a JSON file.
mod file;

/// The `fake` module provides functionalities for generating fake data based on
/// input JSON file and command-line options provided by the user.
mod fake;

/// `main` function, the entry point of the application.
///
/// It executes the `cli::start` function to start the application.
/// If an error occurs during the execution, it is caught and printed to the console.
fn main() {
    match cli::start() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}