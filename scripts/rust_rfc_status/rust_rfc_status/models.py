from dataclasses import dataclass
from typing import Optional, List
from enum import Enum

class Status(str, Enum):
    OPEN = "open"
    CLOSED = "closed"
    MERGED = "merged"
    NOT_FOUND = "not_found"


@dataclass
class Label:
    name: str
    color: str
    link: str
    description: str


@dataclass
class Issue:
    org: str
    repo: str
    number: int
    title: str
    status: Status
    labels: List[Label]
    closed_at: str


@dataclass
class PR:
    org: str
    repo: str
    number: int
    title: str
    status: Status
    labels: List[Label]
    closed_at: str


@dataclass
class RFC:
    sequence: int
    title: str
    file: str
    issue: Optional[Issue]
    pr: Optional[PR]
    features: Optional[List[str]]


