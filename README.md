# Standup

Standup is a command-line tool designed to automate standing desks.

With Standup, users can easily adjust their desk to a sitting or standing position from the terminal without having to press physical buttons. You can also set up a cronjob to make the table move up or down to a schedule.

The standing desk and sensor can be connected via GPIO pins to the computer running the program, e.g. a Raspberry Pi.

## Installation

To use Standup, follow these installation steps:

1. Clone the repository to your local machine:

    ```bash
    git clone https://github.com/ajesipow/standup.git
    cd standup
    cargo build --release
    ```
2. Adapt the [configuration](#configuration).

3. You can run the executable found in the `target/release` directory.


## Configuration

All configuration parameters are defined in `config.toml`.

### Table

1. For the initial setup, manually move your standing desk into the lowest and highest positions, measure the heights and specify `max_table_height` and `min_table_height`.

2. Define the desired `sitting_height` and `standing_height`.

### Motor

Specify the GPIO pin numbers used for driving the table motor up and down.

### Sensor

Specify the GPIO pin numbers connected with the distance sensor for the measurement trigger and echo signal.


### Calibration

The standing desk can calibrate itself for more accurate height measurements.
Calibrating the desk will make it first automatically move all the way up (until `motor.timeout_secs` is reached), take a few measurements, then move all the way down to take more measurements and finally move into the sitting position.

For accurate height estimates, the measurements are normalised with the `table.max_table_height` and `table.min_table_height` values defined above.

Calibration data is stored in a separate file.

## Usage

Standup offers the following commands:

- `calibrate`: Calibrates the standing desk.
- `sit`: Moves the desk to the sitting position.
- `stand`: Moves the desk to the standing position.
- `move-to {height}`: Moves the desk to a specific height.


Example usage:

```bash
# Calibrate the desk with verbose logging
standup calibrate -ddd

# Move the desk to the standing position with less verbose logging
standup stand -dd

# Move the desk to a specific height (e.g., 90 centimeters)
standup move-to 90

# Test the distance sensor
standup test-sensor
```

## License

This project is licensed optionally under either:
* Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or https://opensource.org/licenses/MIT)
