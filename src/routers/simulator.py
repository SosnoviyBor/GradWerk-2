from fastapi import APIRouter, Request

from src.routers.utils.element_parser import create_elements, parse_dist
from src.routers.utils.load_calculator import calculate_load
from src.modeler.model import Model


router = APIRouter()


@router.post("/simulate")
async def simulate(request: Request):
    body = await request.json()
    # full structure of model can be checked in /premade_flowcharts/basic.json
    model = body["model"]
    simtime = float(body["simtime"])
    log_max_size = int(body["log_max_size"])
    assert simtime > 0

    elements = create_elements(model)
    print("Modeling started!")
    simdata = Model(elements).simulate(simtime, log_max_size)
    print("Modeling ended!")

    return simdata


@router.post("/load")
async def load(request: Request):
    body = await request.json()
    data = body["data"]

    load = calculate_load(
        float(data["deviation"]),
        parse_dist(data["dist"]),
        float(data["mean"]),
        int(data["replica"]),
    )

    return load
