from typing import Annotated

from fastapi import Depends, FastAPI, HTTPException, Query
from sqlmodel import Field, Session, SQLModel, create_engine, select


class Person(SQLModel, table=True):
    id: int | None = Field(default=None, primary_key=True)
    name: str = Field(index=True)
    last_name: str | None = Field(default=None, index=True)
    phone_number: str | None= Field(default=None, index=True)
    email_address: str | None = Field(default=None, index=True)
    description: str | None = Field(default=None)

    

