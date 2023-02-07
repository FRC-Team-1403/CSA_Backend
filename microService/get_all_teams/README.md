# Running get_all_teams.py

### Requirements Install
- Using the requirements.txt file, run `pip install -r requirements.txt`

### Modifying Function
The function is already ready to be customized. You just need to modify the `State` and the function will return a list of teams that originate from that state.
Passing in `*` will just return a list of all teams. 

`max_page` limits the amount of teams/requests you make. Normally each page will have ~500 teams and in total there should be `20` pages.

`API_KEY` needs to be changed in order for you to use the function.
```py
# Example Usage
x = get_teams("State", "API_KEY", max_page=20)
print(x)
```
**NOTE:** `State` may have different formats:
- For example: New Jersey is `New Jersey` while Connecticut may be passed in as `CT`.
  - Check the API example to how your state is formatted/abbreviated. 


### Running the File

**Windows**
```shell
python microService/get_all_teams/get_all_teams.py
```

**MacOS/Linux**
```shell
python3 microService/get_all_teams/get_all_teams.py
```