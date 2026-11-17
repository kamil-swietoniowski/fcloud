from fastapi import APIRouter
from fcloud.database import get_session
from fcloud.models.person import Person
from sqlmodel import select

router = APIRouter()

@router.get("/people/", tags=["people"])
async def get_person(session = get_session()) -> list[Person]:
    people = session.exec(select(Person)).all()
    return people

