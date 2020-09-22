import requests
from requests.auth import HTTPBasicAuth
from typing import Optional
from rust_rfc_status.utils import retryable
import os

class BasicFetcher:

    url: str
    response = {}

    auth: HTTPBasicAuth

    def __init__(self, url):
        self.url = url

        github_username = os.getenv("GITHUB_USERNAME")
        github_token = os.getenv("GITHUB_TOKEN")
        self.auth  = HTTPBasicAuth(github_username, github_token)

    @retryable
    def process(self) -> bool:
        print(f"fetching {self.url}")
        res = requests.get(
            self.url, auth=self.auth,
        )
        if not res.ok:
            return False

        self.response = res.json()
        return True

    def bundle(self):
        pass

    def output(self):
        ret = self.process()
        if not ret:
            return None
        
        return self.bundle()