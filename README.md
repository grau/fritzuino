Fritzuino Doorbell
==================
Project to connect a doorbell/pushbutton/whatever controlled by an arduino connected to a pc with a SIP Server like a AVM fritz box.

Whenever the arduino triggers, a SIP call is initiated. 

arduino
-------
The arduino folder contains a tiny plattform.io arduino sketch and and an main.ino file for the arduino part. The arduino waits for a 
high level on port 11 and then prints "RING" on the serial line. Every two seconds it sends a number to indicate it's still alive.

bell-server
-----------
Rust application waiting on serial for "RING" command and initiates a SIP call when received. Can be either run via `cargo run` or
using docker by creating a `bell.env` file from the `bell.env.example` template and starting the application via `docker compose up`.

remarks
-------
This is my first ever project done in rust. If any experienced crustaceans stumbles upon this and finds bad code smells - just let me know.
Always learning.