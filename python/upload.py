import requests

URL = "https://etl-test-higuchi.s3.ap-northeast-1.amazonaws.com/rust-upload/test.txt?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIA4QNCPZ6H5EBMU6XN%2F20250526%2Fap-northeast-1%2Fs3%2Faws4_request&X-Amz-Date=20250526T085111Z&X-Amz-Expires=3600&X-Amz-SignedHeaders=host&X-Amz-Signature=87ea14716ab854bf59fe8263c9d6205ca3ef4a96b377e8db4775595a8eee415d"

FILE_PATH = "./test.txt"

# Open the file in binary mode and upload it to the presigned URL
with open(FILE_PATH, "rb") as file:
    # Upload the file to the presigned URL

    response = requests.put(
        URL,
        data=file
    )

    print("Response status code:", response.status_code)
    if response.status_code == 200:
        print("File uploaded successfully.")
    else:
        print("Failed to upload file. Response:", response.text)
