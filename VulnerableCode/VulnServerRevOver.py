# Vulnerable Python Service

import socket

# Service details
SERVICE_PORT = 9999
BUFFER_SIZE = 256

# Start the vulnerable server
def start_server():
    server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server.bind(("0.0.0.0", SERVICE_PORT))
    server.listen(5)
    print(f"[+] Vulnerable service running on port {SERVICE_PORT}")

    while True:
        client, addr = server.accept()
        print(f"[+] Connection from {addr[0]}:{addr[1]}")
        
        # Send a banner to the client
        client.send(b"Welcome to the vulnerable Python service!\n")
        client.send(f"You can send data up to {BUFFER_SIZE} bytes.\n".encode())

        try:
            # Receive data from the client
            data = client.recv(1024)
            print(f"[*] Received data: {data}")

            # Simulate buffer overflow vulnerability
            if len(data) > BUFFER_SIZE:
                raise OverflowError("Simulated buffer overflow triggered!")

            client.send(b"[+] Data processed successfully!\n")

        except OverflowError as e:
            print(f"[!] {e}")
            client.send(b"[!] Buffer overflow detected!\n")

        except Exception as e:
            print(f"[-] Error: {e}")

        finally:
            client.close()

if __name__ == "__main__":
    try:
        start_server()
    except KeyboardInterrupt:
        print("[!] Shutting down the service.")
