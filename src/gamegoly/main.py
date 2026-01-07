import os
from typing import Optional
from contextlib import asynccontextmanager

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from . import game_config

@asynccontextmanager
async def lifespan(app: FastAPI):
    config_path = getattr(app.state, "config_path", None)
    print(f"CONFIG_PATH: {config_path}")
    if config_path:
        app.state.config = game_config.create_config(config_path)
    yield


def create_app(config_path: Optional[str] = None) -> FastAPI:
    app = FastAPI(lifespan=lifespan)

    app.state.config_path = config_path
    app.state.config = None

    app.add_middleware(
        CORSMiddleware,
        allow_origins=["http://localhost:5173"],
        allow_methods=["*"],
        allow_headers=["*"],
    )

    @app.get("/api/title")
    async def title():
        return {"message": app.state.config.title}

    @app.get("/api/get_tiles")
    async def get_tiles():
        print("here")
        return {"tiles": app.state.config.get_tiles()}

    return app


def parse_args():
    return os.getenv("CONFIG_PATH")


def main():
    config_path = parse_args()
    app = create_app(config_path=config_path)
    return app

app = create_app(parse_args())

if __name__ == "__main__":
    main()
