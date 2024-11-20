from sapota import Sapota, HTTPMETHOD, SapotaCollection

# Create individual HTTP requests
url = "https://api.example.com/data"
headers = [("Content-Type", "application/json"), ("Authorization", "Bearer YOUR_TOKEN")]
method = HTTPMETHOD.POST
body = '{"key1":"value1", "key2":"value2"}'

request1 = Sapota(url, headers, method, body)
request2 = Sapota(url, headers, method, body)

# Print cURL command for a single request
print("Single Request cURL Command:")
print(request1.get_request_command())

# Create a collection of requests
collection = SapotaCollection([request1, request2])

# Export the collection to Postman format
postman_collection_json = collection.export_collection()

# Save the collection to a JSON file
with open("sapota_collection.json", "w") as f:
    f.write(postman_collection_json)

print("\nCollection saved as sapota_collection.json")