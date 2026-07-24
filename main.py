from src.custom_logging import setup_logger
from fastapi import FastAPI
import logging
from src.routs.user import router as user_router
from src.DBManager import DBManager

# Initialize database manager
db_manager = DBManager()
# Setup logger
loger = setup_logger(__name__)
app = FastAPI()
app.include_router(user_router)

@app.get("/")
async def index():
    return {"message": "Home"}