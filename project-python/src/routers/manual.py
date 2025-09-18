import argparse
import json
import os

from src.routers.utils.element_parser import create_elements
from src.modeler.model import Model

# python manual.py --json sample_flowchart.json --simtime 500

if __name__ == "__main__":
    # command line arguments
    parser = argparse.ArgumentParser(description="Run simulation with a local JSON file.")
    parser.add_argument('--json', type=str, required=True, help='JSON file in resources folder')
    parser.add_argument('--simtime', type=float, required=True, help='Simulation time')
    args = parser.parse_args()
    json_filename = args.json
    simtime = args.simtime

    # parse json file
    resources_dir = os.path.join(os.path.dirname(__file__), '..', 'static', 'resources')
    json_path = os.path.abspath(os.path.join(resources_dir, json_filename))
    with open(json_path, 'r', encoding='utf-8') as f:
        model = json.load(f)

    # run simulation
    elements = create_elements(model)
    simdata = Model(elements).simulate(simtime, 0)
