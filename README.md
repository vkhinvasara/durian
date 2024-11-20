# Sapota

`sapota` is a Python library that provides a convenient way to construct and manage HTTP requests using Rust for performance. It supports various HTTP methods and can generate cURL commands for the requests.

## Features

- Construct HTTP requests with various methods (GET, POST, PUT, DELETE, etc.)
- Generate cURL commands for the requests
- Manage collections of HTTP requests
- Export collections to Postman format

## Installation

To install the library, you can use pip:

```sh
pip install sapota
```
## Usage
### Creating a Request
You can create an HTTP request using the Sapota class:

```py
from sapota import Sapota, HTTPMETHOD

url = "https://api.example.com/data"
headers = [("Content-Type", "application/json"), ("Authorization", "Bearer YOUR_TOKEN")]
method = HTTPMETHOD.POST
body = '{"key1":"value1", "key2":"value2"}'

request = Sapota(url, headers, method, body)
command = request.get_request_command()
print(command)
```
## Managing Collections
You can manage collections of HTTP requests using the SapotaCollection class:
```py
from sapota import Sapota, HTTPMETHOD, SapotaCollection

url = "https://api.example.com/data"
headers = [("Content-Type", "application/json"), ("Authorization", "Bearer YOUR_TOKEN")]
method = HTTPMETHOD.POST
body = '{"key1":"value1", "key2":"value2"}'

request1 = Sapota(url, headers, method, body)
request2 = Sapota(url, headers, method, body)
request3 = Sapota(url, headers, method, body)

collection = SapotaCollection([request1, request2, request3])
postman_collection_json = collection.export_collection()
print(postman_collection_json)

with open("sapota_collection.json", "w") as f:
    f.write(postman_collection_json)
print("Collection saved as sapota_collection.json")
```

## License
This project is licensed under the MIT License.