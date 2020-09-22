import json
import os
import pathlib
import re
from dataclasses import dataclass, is_dataclass, asdict
from enum import Enum
from typing import Optional, List
from zipfile import ZipFile

import requests
from requests.auth import HTTPBasicAuth

import functools

from rust_rfc_status.RfcParser import RfcParser
from rust_rfc_status.utils import retryable

class EnhancedJSONEncoder(json.JSONEncoder):
    def default(self, o):
        if is_dataclass(o):
            return asdict(o)
        return super().default(o)


@retryable
def download_latest_rfc_files():
    master_zip = "https://github.com/rust-lang/rfcs/archive/master.zip"
    local_file = master_zip.split("/")[-1]
    with requests.get(master_zip, stream=True) as r:
        r.raise_for_status()
        with open(local_file, "wb") as f:
            for chunk in r.iter_content(chunk_size=8192):
                f.write(chunk)

    with ZipFile(local_file, "r") as zf:
        zf.extractall("rfc")


def get_rfc_detail():
    rfc_folder = pathlib.Path("./rfc/rfcs-master/text")
    rfc_files = os.listdir(rfc_folder)
    rfc_files.sort()
    rfc_detail = {}

    for rfc in rfc_files:
        a_path = rfc_folder.joinpath(rfc)
        parser = RfcParser(a_path)
        detail = parser.output()

        rfc_detail[detail.sequence] = detail
        
    return rfc_detail



def notify_if_status_update():
    pass


def fetch_rfc_status():
    rfc_detail = get_rfc_detail()

    j = json.dumps(rfc_detail, indent=4, cls=EnhancedJSONEncoder)
    with open("latest.json", "w") as fd:
        fd.write(j)

    notify_if_status_update()


def main():
    download_latest_rfc_files()
    fetch_rfc_status()




if __name__ == "__main__":
    main()
