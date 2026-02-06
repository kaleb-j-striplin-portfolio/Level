# Level
Rust implementation of a bubble level for the Microbit V2

It uses the 5x5 LED array, A button, B button, timer, and accellerometer

### Author

Kaleb Striplin

### Run

Plug in your MicroBit v2

```rust
cargo embed --release
``` 

### Specs 

* The program updates the display every 200 ms (5 frames per second)

* The lit LED acts as the bubble in an analog level. For example as the left side of the board tilts up, the led lights up closer to the left side.

* The LEDs will all be off when the board detects it is upside down. (positive z)

* The program begins in coarse mode where it moves the level evenly accross a range of -500 mg to 500 mg in x and y. The accellerometer is put in low power mode.

* Clicking the B button will switch to fine mode where the range is reduced to -50 mg to 50 mg. The accellerometer is put in high resolution mode.

* Clicking the A button will switch to coarse mode with the original range of -500 mg to 500 mg and the accellerometer in low power mode.

### Acknowledgements

Starting code for initializing the accellerometer on the mb2 was provided by the Rust Embedded MB2 Discovery Book. https://docs.rust-embedded.org/discovery-mb2/

Specifically chapter 12 on I2C has sample code for reading the accellerometer

### License

MIT, see LICENSE