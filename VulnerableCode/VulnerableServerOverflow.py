import os
import socket

file_dir = '/tmp/vulnerable_app'

if not os.path.exists(file_dir):
    os.makedirs(file_dir)


server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server_socket.bind(('localhost', 8080))
server_socket.listen(1)

print('Server listening on port 8080...')

while True:
    client_socket, address = server_socket.accept()
    print('Connection from', address)

    try:
        request = client_socket.recv(1024)
        print(f"Request received: {request.decode('utf-8', errors='ignore')}")

        if request.startswith(b'UPLOAD'):
            print("Simulating buffer overflow...")

            buffer = bytearray(1024) 
            data = client_socket.recv(2048)  # Unsafe: more than the buffer size
            try:
                buffer[:len(data)] = data  #
                print("Data written to buffer.")
            except ValueError as e:
                print(f"Buffer overflow detected! {e}")
                client_socket.send(b'Buffer overflow detected!')

        elif request.startswcdith(b'DOWNLOAD'):
            file_name = request[9:].decode('utf-8').strip()
            file_path = os.path.join(file_dir, file_name)
            if os.path.exists(file_path):
                with open(file_path, 'rb') as f:
                    while chunk := f.read(1024):
                        client_socket.send(chunk)
                print('File downloaded:', file_name)
            else:
                client_socket.send(b'File not found')

        elif request.startswith(b'EXEC'):
            command = request[5:].decode('utf-8').strip()
            os.system(command)
            print('Command executed:', command)
        else:
            client_socket.send(b'Invalid request')
    except Exception as e:
        print(f"Error: {e}")
        client_socket.send(f"Error: {e}".encode('utf-8'))

    client_socket.close()
