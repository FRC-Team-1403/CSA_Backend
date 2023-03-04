import requests
from typing import List


def get_teams(
        state_key: str,
        api_key: str,
        max_page: int = 20,
        url: str = "https://www.thebluealliance.com/api/v3/teams/{}"
) -> List[str]:
    """
    Get all teams in a state

    :param state_key: A team's state to filter based on. You can also pass in `*` to just return everything.
    :param api_key: API Key to use to query the base URL
    :param max_page: The maximum page to stop at.
    :param url: The base URL to query at.
    :return: "List of team numbers"
    """
    team_keys = []
    for page in range(0, max_page):
        response = requests.get(url.format(page), headers={"X-TBA-Auth-Key": api_key})
        if response.status_code == 200:
            teams = response.json()
            for team in teams:
                if state_key != "*":
                    if team.get("state_prov") == state_key:
                        team_keys.append(team.get("team_number"))
                else:
                    team_keys.append(team.get("team_number"))
        else:
            pass
    return team_keys


def get_team_competing(
        event_key: str,
        api_key: str,
        max_page: int = 20,
        url: str = "https://www.thebluealliance.com/api/v3/event/{}/teams"
) -> List[str]:
    """
    Get all teams in a state

    :param event_key: A team's state to filter based on. You can also pass in `*` to just return everything.
    :param api_key: API Key to use to query the base URL
    :param max_page: The maximum page to stop at.
    :param url: The base URL to query at.
    :return: "List of team numbers"
    """
    response = requests.get(url.format(event_key), headers={"X-TBA-Auth-Key": api_key})
    teams = []
    if response.status_code == 200:
        team = response.json()
        for team in team:
            if team.get("team_number"):
                teams.append(str(team.get("team_number")))
        return ", ".join(teams)
    else:
        return []


"""
Example Usage
x = get_teams("State", "API_KEY")
print(x)
"""
