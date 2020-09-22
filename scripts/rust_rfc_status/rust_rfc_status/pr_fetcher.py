from rust_rfc_status.basic_fetcher import BasicFetcher
from rust_rfc_status.models import PR, Status, Label
from typing import List

class PullFetcher(BasicFetcher):
    
    org: str
    repo: str
    number: int

    title: str
    status: Status
    labels: List[Label]
    closed_at: str

    def __init__(self, org, repo, number):
        self.org =org
        self.repo = repo
        self.number = number
        url = f"https://api.github.com/repos/{org}/{repo}/pulls/{number}"
        super().__init__(url)
    

    def get_detail(self):
        self.title = self.response["title"]

        self.status = Status(self.response["state"])

        if self.response.get("merged", None) in [True, None]:
            self.status = Status.MERGED

        self.labels = [
                Label(
                    name=label["name"],
                    color=label["color"],
                    link=label["url"],
                    description=label["description"],
                )
                for label in self.response["labels"]
            ]
            
        self.closed_at = self.response["closed_at"]


    def bundle(self) -> PR:
        self.get_detail()
        
        return PR(self.org, self.repo, self.number, self.title, self.status, self.labels, self.closed_at)
