from fastapi import FastAPI
import fcloud_core 
import uvicorn
from fcloud.routers import person

app = FastAPI(title="Fcloud API")

app.include_router(person.router)


@app.get("/")
def health_check():
    return {"status": "online"}

@app.get("/test/{a}/{b}")
def test_rust(a: int, b: int):
    wynik = fcloud_core.sum_as_string(a, b)
    return {"wynik": wynik}


def main():
    uvicorn.run("fcloud.main:app", host="127.0.0.1", port=8000, reload=True)
