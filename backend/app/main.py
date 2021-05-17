import datetime

import envir
import httpx
import motor.motor_asyncio
from aiogram import Bot, types
from fastapi import FastAPI
from fastapi_utils.tasks import repeat_every

TELEGRAM_BOT_SECRET_KEY = envir.load("TELEGRAM_BOT_SECRET_KEY")
TELEGRAM_RESOURCE_CHANNEL = envir.load("TELEGRAM_RESOURCE_CHANNEL")
MONGO_URL = envir.load("MONGO_URL")

app = FastAPI()
motor_client = motor.motor_asyncio.AsyncIOMotorClient(MONGO_URL)
telegram_bot = Bot(token=TELEGRAM_BOT_SECRET_KEY, parse_mode=types.ParseMode.MARKDOWN_V2)


@app.on_event("startup")
@repeat_every(seconds=60 * 20)
async def fetch_reddit_data() -> None:
    print("Fetching....")
    db = motor_client['resource']
    collection = db['reddit']
    async with httpx.AsyncClient() as client:
        headers = {'user-agent': 'rust:resource:v0.1.0 (by /u/kilerd_chan)'}
        r = await client.get("https://www.reddit.com/r/rust/.json", headers=headers)
        res_data = r.json()
        post_items = [{
            "id": item['data']['id'],
            "score": item['data']['score'],
            "title": item['data']['title'],
            "selftext": item['data']['selftext'],
            "author": item['data']['author'],
            "permalink": item['data']['permalink'],
            "url": item['data']['url'],
            "create_time": datetime.datetime.now()
        } for item in res_data['data']['children'] if item['data']['score'] >= 50]

        for item in post_items:
            n = await collection.count_documents({'id': {'$eq': item['id']}})
            # if n > 0:
            #     break
            print(f"insert reddit item id:{item['id']} ")
            permalink = f"https://www.reddit.com{item['permalink']}"
            title = f"\\[[Reddit]({permalink})\\] *{item['title']}*"
            content = ""
            for line in item.get("selftext", "").splitlines():
                if len(content) > 350:
                    break
                content = f"{content}\n{line}"
            if content != "":
                content = f"\n\n{content}"

            if item['url'] != permalink and item['url'] != item['permalink']:
                content = f"{content}\n\n{item['url']}"
            try:
                msg = await telegram_bot.send_message(TELEGRAM_RESOURCE_CHANNEL, f"{title}{content}",
                                                      disable_web_page_preview=True)

                item['msg_id'] = msg.message_id
                result = await collection.insert_one(item)
            except Exception as ex:
                print(f"cannot send to telegram. title: {title}, e: {ex}")


@app.get("/")
def read_root():
    return {"Hello": "World"}


@app.get("/delete")
async def read_item():
    db = motor_client['resource']
    collection = db['reddit']
    await collection.delete_many({})
    return {}
@app.get("/trending/go")
async def read_item():
    await fetch_reddit_data()
    return {}


@app.get("/trending")
async def read_item():
    db = motor_client['resource']
    collection = db['reddit']
    items = await collection.find({}, {'_id': 0}).sort('_id', -1).limit(50).to_list(length=50)
    return {
        'data': items
    }
