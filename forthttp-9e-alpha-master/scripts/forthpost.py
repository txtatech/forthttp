import requests

def send_forth_code_to_server(code):
    url = 'http://127.0.0.1:8080/forth'
    headers = {'Content-Type': 'application/text'}
    response = requests.post(url, headers=headers, data=code)
    return response.text

# Usage:
forth_code = "+ 1 2"  # Replace this with your actual Forth code
result = send_forth_code_to_server(forth_code)
print(result)
