from rust_rfc_status.models import RFC, Issue, PR
from typing import Dict, List, Optional
from rust_rfc_status.issue_fetcher import IssueFetcher
from rust_rfc_status.pr_fetcher import PullFetcher
import re

class RfcParser:
    file: str

    sequence: str
    title: str
    extra: Dict[str, str] = {}


    def __init__(self, file):
        self.file = file


    def process(self):
        
        with open(self.file) as rfc_fd:
            self.file  = str(self.file).split("/")[-1]
            self.sequence = self.file.split("-")[0]
            self.title = self.file.split("-", 1)[1].replace("-", " ")[:-3]
            lines = rfc_fd.readlines(1024 * 8)
            for line in lines:
                if line.startswith("- "):
                    data = line.split(":", maxsplit=1)
                    field_name = data[0][2:].replace("#", "").strip()
                    self.extra[field_name] = data[1].strip()
                else:
                    break
    
    def fetch_issue(self) -> Optional[Issue]:
        value  = self.extra.get("Rust Issue", "")
        search = re.match(r"\[([a-zA-z0-9\-]+)/([a-zA-z0-9\-]+)#(\d+)\]", value)
        if search:
            org = search.group(1)
            repo = search.group(2)
            number = int(search.group(3))
            fetcher = IssueFetcher(org,repo, number)
            return fetcher.output()
        
        return None

    def fetch_pr(self) -> Optional[Issue]:
        value  = self.extra.get("RFC PR", "")
        search = re.match(r"\[([a-zA-z0-9\-]+)/([a-zA-z0-9\-]+)#(\d+)\]", value)
        if search:
            org = search.group(1)
            repo = search.group(2)
            number = int(search.group(3))
            fetcher = PullFetcher(org,repo, number)
            return fetcher.output()
        
        return None

    def get_features(self) -> List[str]:
        feature_name = self.extra.get("Feature Name", "")
        search = re.findall(r"`([a-zA-z0-9\-]+)`", feature_name, re.DOTALL)
        return search

    def output(self) -> RFC:
        self.process()
        issue = self.fetch_issue()
        pr = self.fetch_pr()
        features = self.get_features()
        return RFC(self.sequence, self.title, self.file, issue, pr, features)