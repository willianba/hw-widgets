# hw-widgets

this is my attempt of implementing a series of widgets to show my hardware information on my secondary monitor.

i was using aida64 previously and i discovered it was causing some performance issues when gaming. i couldn't find any app close to aida, so i decided to implement my own while learning new technologies.

this is not meant to be reusable or compatible with anything but my system. if i ever make this work on my pc the way i want, i can start thinking about expanding it later.

## tech stack

> this is my first rust app ever. expect bad code ahead.

this repo uses tauri + yew. it's meant to be something to explore rust and learn how to use it.

### dependencies

to get the hardware information from sensors, i'm relying on [HWiNFO](https://www.hwinfo.com/). this app can read hardware information from the pc sensors and expose them easily to the system.

the end goal is to run this app on my windows pc to see resources while gaming. so windows it's another dependency as i rely on its registry to collect the sensors data.

### hwinfo

> TODO add necessary hwinfo setup steps here

## running the code

running `cargo tauri dev` should download all the dependencies and start the app in development mode.

of course, make sure you have rust and cargo installed. if you don't, check [rustup](https://www.rust-lang.org/tools/install) to install it.

## inspiration

i'm not a good frontend developer, but my goal is to have something as beautiful as the screens you can build with [eww](https://github.com/elkowar/eww).
