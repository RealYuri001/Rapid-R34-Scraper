import asyncio

import aiohttp

async def main():
    ses = aiohttp.ClientSession()
    resp = await ses.get("https://api.rule34.xxx/index.php?page=dapi&s=post&q=index&json=1&limit=1")
    
    if resp.ok:
        print(await resp.json())
    
    await ses.close() #Always close the session.
    
asyncio.run(main())