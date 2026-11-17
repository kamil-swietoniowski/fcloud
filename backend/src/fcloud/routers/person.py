from fastapi import APIRouter

router = APIRouter()

@router.get("/people/", tags=["people"])
async def get_person():
    return "Hello World!"