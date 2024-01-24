
from poe_api_wrapper import PoeApi
from enum import Enum
import toml

client      = None

class Bots(Enum):
    CLAUDE_INSTANT_100K = "a2_2"
    CLAUDE_2_100K = "a2"
    ALIENOR = "AlienorGPT"
    DALLE   = "dalle3"

class Thread():

    def __init__(self, bot: Bots = Bots.CLAUDE_2_100K):

        self.bot     = bot
        self.chat_id = None
        self.deleted = False

    def generate_message(self, message: str) -> str:

        if self.deleted:
            raise Exception("Thread has been deleted")

        token_stream = client.send_message(self.bot.value, message, chatId=self.chat_id)
        
        for chunk in token_stream:
            pass

        self.chat_id = chunk["chatId"]
        return chunk["text"]
    
    def delete_chat(self):

        client.delete_chat(self.chat_id, chatId=self.chat_id)
        self.deleted = True

def generate_message(message: str, bot: Bots = Bots.CLAUDE_2_100K) -> str:

    for chunk in client.send_message(bot.value, message):
        pass

    client.delete_chat(bot.value, chatId=chunk["chatId"])
    return chunk["text"]

def dalle_trial():

    token_stream = client.send_message("dalle3", "Generate a picture of a realistic depiction of a trial")
    for chunk in token_stream:
        pass
    t_chat_id = chunk["chatId"]
    print(chunk["text"])
    client.delete_chat("dalle3", chatId=t_chat_id)

def init_poe_wrapper():

    global client
    client = PoeApi(toml.load("./settings.toml")["poe_api_key"])