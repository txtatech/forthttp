import requests

url = "http://127.0.0.1:8080/forth"
headers = {"Content-Type": "text/plain"}
data = "3 4 + ."

response = requests.post(url, headers=headers, data=data)

print("Status code: ", response.status_code)
print("Response: ", response.text)
