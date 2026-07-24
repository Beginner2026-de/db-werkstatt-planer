# users.py
from fastapi import APIRouter

router = APIRouter(
    prefix="/users",  # Optional: Ein gemeinsames Präfix für alle Routen hier
    tags=["users"]    # Optional: Für die Dokumentation (Swagger UI)
)

@router.get("/")
def get_users():
    return {"massege": "home user"}

