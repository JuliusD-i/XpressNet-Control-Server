import serial

def read_from_serial(port, baud_rate):
    try:
        with serial.Serial(port, baud_rate, timeout=1) as ser:
            print(f"Connected to {port} at {baud_rate} baud.")
            while True:
                if ser.in_waiting > 0:
                    data = ser.read(ser.in_waiting)
                    for byte in data:
                        command = byte & 0b01100000
                        if command == 0b1000000:
                            device_address = byte & 0b00011111  # Mask out the parity bit (MSB)
                            binary_address = format(device_address, '05b')
                            decimal_address = device_address
                            print(f"Binary: {binary_address} Decimal: {decimal_address}")
                            
    except serial.SerialException as e:
        print(f"Error opening or reading from serial port: {e}")
    except KeyboardInterrupt:
        print("Exiting program.")

if __name__ == "__main__":
    port = "/dev/ttyUSB0" #input("Enter the serial port (e.g., COM3 or /dev/ttyUSB0): ")
    baud_rate = 62500 #int(input("Enter the baud rate (e.g., 9600): "))
    read_from_serial(port, baud_rate)
