import boto3
from botocore.exceptions import NoCredentialsError
from datetime import datetime, timedelta
from botocore.client import Config

# https://dev.classmethod.jp/articles/s3-presigned-url-signature-mismatch-after-bucket-creation/

BUCKET_NAME = "etl-test-higuchi"
KEY = "rust-upload/test.txt"  # ファイル名を指定


def generate_presigned_url(bucket_name, key, expiration=3600):
    """
    Generate a presigned URL for PUT operation on S3.

    :param bucket_name: Name of the S3 bucket
    :param key: Key (path) in the S3 bucket
    :param expiration: Time in seconds for the presigned URL to remain valid
    :return: Presigned URL as a string
    """
    s3_client = boto3.client("s3",
                             region_name="ap-northeast-1",
                             config=Config(
                                 signature_version='s3v4', s3={'addressing_style': 'virtual'}))

    try:
        url = s3_client.generate_presigned_url(
            ClientMethod="put_object",
            Params={"Bucket": bucket_name, "Key": key},
            ExpiresIn=expiration,
            HttpMethod="PUT"
        )
        return url
    except NoCredentialsError:
        print("AWS credentials not found.")
        return None


if __name__ == "__main__":
    presigned_url = generate_presigned_url(BUCKET_NAME, KEY)
    if presigned_url:
        print(presigned_url)
    else:
        print("Failed to generate presigned URL.")
