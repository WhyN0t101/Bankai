import os
import socket

# Set up the file directory
file_dir = '/tmp/vulnerable_app'

# Create the file directory if it doesn't exist
if not os.path.exists(file_dir):
    os.makedirs(file_dir)

# Set up the socket server
server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server_socket.bind(('localhost', 8080))
server_socket.listen(1)

print('Server listening on port 8080...')

while True:
    # Accept incoming connections
    client_socket, address = server_socket.accept()
    print('Connection from', address)

    # Handle incoming requests
    request = client_socket.recv(1024)
    if request.startswith(b'UPLOAD'):
        # Handle file upload
        file_name = request[7:].decode('utf-8')
        file_path = os.path.join(file_dir, file_name)
        with open(file_path, 'wb') as f:
            while True:
                chunk = client_socket.recv(1024)
                if not chunk:
                    break
                f.write(chunk)
        print('File uploaded:', file_name)
    elif request.startswith(b'DOWNLOAD'):
        # Handle file download
        file_name = request[9:].decode('utf-8')
        file_path = os.path.join(file_dir, file_name)
        if os.path.exists(file_path):
            with open(file_path, 'rb') as f:
                while True:
                    chunk = f.read(1024)
                    if not chunk:
                        break
                    client_socket.send(chunk)
            print('File downloaded:', file_name)
        else:
            client_socket.send(b'File not found')
    elif request.startswith(b'EXEC'):
        # Handle system command execution
        command = request[5:].decode('utf-8')
        os.system(command)
        print('Command executed:', command)
    else:
        client_socket.send(b'Invalid request')

    # Close the client socket
    client_socket.close()
