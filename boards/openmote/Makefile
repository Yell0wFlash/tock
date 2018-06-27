# Makefile for building the tock kernel for the nRF development kit

TOCK_BOARD=openmote
TOCK_ARCH=cortex-m3
TARGET=thumbv7m-none-eabi

PLATFORM=openmote
#path to the flashing script
BSL_BASE=/Users/admin/School/Ex-Jobb/cc2538-bsl-master
#mote port
PORT=/dev/tty.usbserial-14401

include ../Makefile.common


flash: target/$(TARGET)/release/openmote.bin
	@echo "Flashing target/$(TARGET)/release/openmote.bin"
	python $(BSL_BASE)/cc2538-bsl.py -e -w -v -a 0x00200000 -p $(PORT) --bootloader-invert-lines target/$(TARGET)/release/openmote.bin

