import requests
from typing import List


def get_event_name(
        event_key: str,
        api_key: str,
        url: str = "https://www.thebluealliance.com/api/v3/event/{}"
) -> List[str]:
    """
    Get all teams in a state

    :param event_key: A team's state to filter based on. You can also pass in `*` to just return everything.
    :param api_key: API Key to use to query the base URL
    :param url: The base URL to query at.
    :return: "List of team numbers"
    """
    response = requests.get(url.format(event_key), headers={"X-TBA-Auth-Key": api_key})
    if response.status_code == 200:
        event = response.json()
        if event.get("name"):
            return event.get("name")
        else:
            return []
    else:
        return []
