import requests
from bs4 import BeautifulSoup
base_url = "https://conwaylife.com"
wiki_url = f"{base_url}/wiki/Category:Lists_of_patterns"


def get_patterns_links() -> list[str]:
    response = requests.get(wiki_url).text
    soup = BeautifulSoup(response, "html.parser")
    patterns_list = soup.find("div", id ="mw-pages")
    links = patterns_list.find_all('a')
    return [ link.get("href") for link in links]

def parse_pattern(link:str):
    response = requests.get(link)
    #TODO


if __name__ == "__main__":
    for link in get_patterns_links():
        parse_pattern(f"{base_url}{link}")