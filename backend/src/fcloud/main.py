from fastapi import FastAPI
import fcloud_core

app = FastAPI(title="Fcloud API")

@app.get("/")
def health_check():
    return {"status": "online"}

@app.get("/test/{a}/{b}")
def test_rust(a: int, b: int):
    # Wywołanie funkcji z Twojego pakietu fcloud_core
    wynik = fcloud_core.sum_as_string(a, b)
    return {"wynik": wynik}
