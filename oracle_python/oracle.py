import near_api
import json
import os
import time
import requests
import datetime

near_provider = near_api.providers.JsonProvider(
    "https://rpc.mainnet.near.org")
contract_id = "language.betteryou.near"

# Gets all the users that participate in the challenge.


def get_all_users():
    result = near_provider.view_call(
        contract_id, "get_all_state", json.dumps({}).encode('utf8'))
    all_users = json.loads(''.join([chr(x) for x in result['result']]))
    result = {}
    for user in all_users:
        user_info = user[1]
        # Look only on the active challenges
        if user_info['days_left'] > 0 and user_info['lives_left'] > 0:
            register_seconds = user_info['register_timestamp'] / 1000000000
            # If the day has passed, we should update (this is the case where user didn't update their duolingo)
            should_refresh = register_seconds + \
                (user_info['days_passed'] + 1) * 86400 < time.time()
            result[user[0]] = {'should_refresh': should_refresh, 'total_xp': user_info['total_xp'],
                               'duolingo_username': user_info['duolingo_username'], 'language': user_info['language']}
    return result

# Fetch the XP points from duolingo for a given user.


def fetch_duolingo(username, language, near_account):
    response = requests.get(
        f"https://www.duolingo.com/2017-06-30/users?username={username}")
    if not response.ok:
        print(
            f"Failed to fetch {near_account} {username} {language} : {response}")
        return None
    try:
        for course in response.json()['users'][0]['courses']:
            if course['title'] == language:
                return course['xp']
        print(
            f"Failed to find the language {language} for {username} NEAR: {near_account}")
    except Exception as e:
        print(
            f"Failed to fetch {username} {language} NEAR: {near_account}: {e}")
    return None


def find_users_to_refresh():
    all_users = get_all_users()
    to_refresh = []
    for k, v in all_users.items():
        xp = fetch_duolingo(v["duolingo_username"], v["language"], k)
        should_refresh = v["should_refresh"]
        if xp and xp != v["total_xp"]:
            should_refresh = True
        else:
            print(f"User {k} - no XP update")
        if should_refresh:
            to_refresh.append((k, xp or 0))
    return to_refresh


# Signer private key (in format ed25519:xxxxx)
# signer_key = os.environ["NEAR_SIGNER_KEY"]


def update_users_in_protocol(to_refresh):
    signer_id = "language.betteryou.near"
    signer_key = os.environ["NEAR_SIGNER_KEY"]
    args = {"update": to_refresh}

    key_pair = near_api.signer.KeyPair(signer_key)
    signer = near_api.signer.Signer(signer_id, key_pair)
    account = near_api.account.Account(near_provider, signer, signer_id)

    out = account.function_call(contract_id, "admin_update_challenge", args)

    print(out)


while True:
    to_refresh = find_users_to_refresh()

    print(f"{datetime.datetime.now()} - users to update: {to_refresh}")
    if to_refresh:
        try:
            update_users_in_protocol(to_refresh)
        except Exception as e:
            print(f"Failed to update blockchain: {e}")
    # Sleep 10 minutes
    time.sleep(60*10)
