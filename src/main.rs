#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};                                   
use panic_rtt_target as _;

use embedded_hal::digital::InputPin;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{
        timer::Timer,
        twim,
    },
    pac::twim0::frequency::FREQUENCY_A,
};

use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};

fn generate_bubble_display(x: usize, y: usize) -> [[u8; 5]; 5]  {
    let mut display = [[0; 5]; 5];
    display[y][x] = 1;
    display
}

#[entry]
fn init() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();

    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
    let mut timer = Timer::new(board.TIMER0);
    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;
    let mut display: Display = Display::new(board.display_pins);


    // Code from documentation
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor
        .set_accel_mode_and_odr(
            &mut timer,
            AccelMode::LowPower,
            AccelOutputDataRate::Hz50,
        )
        .unwrap();

    let frame_refresh_ms: u32 = 200;
    // Start in "coarse" mode: divide the range from -500 to 500 mG into 5 parts.
    let mut range: i32 = 500;
    let mut bubble: [[u8; 5]; 5];
    loop {
        if sensor.accel_status().unwrap().xyz_new_data() {
            let (x, y, z) = sensor.acceleration().unwrap().xyz_mg();
            // RTT instead of normal print
            rprintln!("Acceleration: x {} y {} z {} mg", x, y, z);

            let pressed_a: bool = button_a.is_low().unwrap();
            let pressed_b:bool = button_b.is_low().unwrap();

            // Pressing the B button (by itself) should put the level in "fine" mode: now the LED scales should go from -50 to 50 mG.
            if pressed_b && !pressed_a {
                rprintln!("switching to fine mode: (-50mg to 50mg)");
                range = 50;
                if sensor.get_accel_mode() != AccelMode:: HighResolution {
                    rprintln!("switching to high resolution mode");
                    sensor.set_accel_mode_and_odr(
                        &mut timer,
                        AccelMode::HighResolution,
                        AccelOutputDataRate::Hz50,
                    )
                    .unwrap(); 
                }
            }

            // Pressing the A button (by itself) should return to "coarse" mode.
            if pressed_a && !pressed_b {
                rprintln!("switching to coarse mode: (-500 mg to 500 mg)");
                range = 500;
                if sensor.get_accel_mode() != AccelMode:: LowPower {
                    rprintln!("switching to low power mode");
                    sensor.set_accel_mode_and_odr(
                        &mut timer,
                        AccelMode::LowPower,
                        AccelOutputDataRate::Hz50,
                    )
                    .unwrap(); 
                } 
            }

            // When the board is upside down (z is positive), the display should be blanked.
            if z > 0 {
                bubble = [[0; 5]; 5];
            }
            else {
                let step = (range * 2) / 5;
                rprintln!("step: {}", step);

                let x_position: usize = match x {
                    x if x > range + -step => 0,
                    x if x <= range + -step && x > range + (step * -2) => 1,
                    x if x <= range + (step * -2) && x > range + (step *  -3) => 2, 
                    x if x <= range + (step * -3) && x > range + (step *  -4) => 3, 
                    x if x <= range + (step * -4)=> 4,
                    _ => 0
                };

                let y_position: usize = match y {
                    y if y > range + -step => 4,
                    y if y <= range + -step && y > range + (step * -2) => 3,
                    y if y <= range + (step * -2) && y > range + (step *  -3) => 2, 
                    y if y <= range + (step * -3) && y > range + (step *  -4) => 1, 
                    y if y <= range + (step * -4)=> 0,
                    _ => 0
                };
                bubble = generate_bubble_display(x_position, y_position);
            }
            display.show(&mut timer,  bubble, frame_refresh_ms);
        }
    }
}
