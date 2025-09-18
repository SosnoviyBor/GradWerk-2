import json


def decode(data: bytes) -> dict:
    # i dont know why request encodes these symbols, but fuck me i guess
    result = json.loads(
        data.decode()
        .replace("result=", "")
        # results replacers
        .replace("%5B", "[")
        .replace("%5D", "]")
        .replace("%7B", "{")
        .replace("%7D", "}")
        .replace("%22", '"')
        .replace("%3A", ":")
        .replace("%2C", ",")
        .replace("+", " ")
        # log replacers
        .replace("%5C", "\\")
        .replace("%3E", ">")
        .replace("%3C", "<")
        .replace("%23", "#")
        .replace("%7C", "|")
        .replace("%3D", "=")
        .replace("%21", "!")
    )
    result["log"]["first"] = [log.split("\n") for log in result["log"]["first"]]
    result["log"]["last"] = [log.split("\n") for log in result["log"]["last"]]

    return result
