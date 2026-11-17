from fastapi import APIRouter, Depends
from fcloud.database import get_session
from fcloud.models.person import Person
from sqlmodel import select, Session

router = APIRouter()

@router.get("/people/", tags=["people"])
async def get_person(session: Session = Depends(get_session)) -> list[Person]:
    people = session.exec(select(Person)).all()
    return people

@router.post("/people/", tags=["people"])
async def create_person(person: Person, session: Session = Depends(get_session)):
    session.add(person)
    session.commit()
    session.refresh(person)
    return person

