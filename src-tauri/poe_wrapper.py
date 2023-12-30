
from poe_api_wrapper import PoeApi
import toml

poe_api_key = None
client      = None
chat_id     = None

def generate_message(message):

    global chat_id

    token_stream = None

    if chat_id is None:
        token_stream = client.send_message("AlienorGPT", message)
    else:
        token_stream = client.send_message("AlienorGPT", message, chatId = chat_id)

    for chunk in token_stream:
        pass

    if chat_id is None:
        chat_id = chunk["chatId"]

    return chunk["text"]

def dalle_trial():

    token_stream = client.send_message("dalle3", "Generate a picture of a realistic depiction of a trial")
    for chunk in token_stream:
        pass
    t_chat_id = chunk["chatId"]
    print(chunk["text"])
    client.delete_chat("dalle3", chatId=t_chat_id)

def init():

    global poe_api_key
    global client

    poe_api_key = toml.load("./Settings.toml")["poe_api_key"]
    client = PoeApi(poe_api_key)
    dalle_trial()

init()