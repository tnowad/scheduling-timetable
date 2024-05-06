# Class Scheduling Application

This is a Rust application that generates class schedules based on a provided
JSON file containing class information, subject codes, and days of the week.

## Features

- Reads class data from a JSON file
- Filters classes based on specified subject codes and days of the week
- Generates all possible schedules that satisfy the given constraints
- Prints the generated schedules in a tabular format

## Usage

- Ensure you have Rust installed on your system.
- Clone this repository or download the source code.
- Navigate to the project directory.
- Place your JSON file containing class data in the project directory.
  The default file name is data.json, but you can modify the code to use a
  different file name. (you can find data from here `https://{website}/api/dkmh/w-locdsnhomto`)
- Compile and run the project using the following command:

  ```bash
  cargo run
  ```

- When prompted, enter the subject codes and days of the week (2-7, where 2 is
  Monday and 7 is Sunday) you want to include in the schedule generation.
- The application will generate all possible schedules and print them in a tabular
  format.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements,
please open an issue or submit a pull request.

## Disclaimer

This application is for educational purposes only and should not be used for
any commercial or illegal activities. The author(s) of this application are not
responsible for any misuse or consequences arising from the use of this application.
