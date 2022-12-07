# rust_rpi_sample
Basic sample Rust application to blink LED in Raspberry Pi.

Source is developed in macOS and cross compiled for the target *aarch64-unknown-linux-gnu*

## Setup macOS
```
$ cd <project_directory>
$ chmod +x setup.sh
$ ./setup.sh
```
setup.sh will prompt for the **macOS password** to install the target gcc

## Cross compile and Deployment on Raspberry Pi
Setup Raspberry Pi with wifi/LAN
```
$ chmod +x deploy.sh
$ ./deploy.sh <raspberry_pi_ip>
```
deploy.sh will prompt for the **Raspberry Pi password** to scp the executable

## Run the application in Raspberry Pi
Access Raspberry Pi command prompt using ssh
```
ssh pi@<raspberry_pi_ip>
```
You will be prompt for the **Raspberry Pi password**

Run the application
```
$ chmod +x /home/pi/test/rpi_sample
$ ./rpi_sample
```
