import json

from flask import Flask, request, make_response, send_from_directory, current_app, send_file, jsonify
from flask_restx import Api, fields, Resource

from _1D.cellular_automata import generate_random, generate_rule, cellular_automata_step_1d
from _2D.general_2d_automata import generate_grid_random_cells, generate_grid_central, update_grid_one_d, rules
from utils.utils import RoundList

api = Api(title='Cellular automata', default="CellularAutomata")
app = Flask(__name__)
api.init_app(app)


@app.after_request
def after_request(response):
    response.headers.add('Access-Control-Allow-Origin', '*')
   # response.headers.add('Access-Control-Allow-Headers', 'Content-Type,Authorization')
    response.headers.add('Access-Control-Allow-Headers', '*')
    response.headers.add('Access-Control-Allow-Methods', 'GET,PUT,POST,DELETE')
    return response


@api.route('/grid/2d/random')
class Grid2dRandom(Resource):
    @api.doc(params={'width': 'width', 'height': 'height', 'one_prob': 'probability of one (colored state)'})
    def get(self):
        print(request.args)
        width = request.args.get('width')
        height = request.args.get('height')
        probability_of_one = request.args.get('one_prob')
        grid = generate_grid_random_cells(width=int(width),
                                          height=int(height),
                                          probability_of_one=float(probability_of_one))
        response = jsonify({'grid': grid})
        return response


@api.route('/grid/2d/center')
class Grid2dCenter(Resource):
    @api.doc(params={'width': 'width', 'height': 'height', 'cell_count': 'number of cells in grid center'})
    def get(self):
        print(request.args)
        width = request.args.get('width')
        height = request.args.get('height')
        cell_count = request.args.get('cell_count')
        grid = generate_grid_central(width=int(width),
                                     height=int(height),
                                     cell_count=int(cell_count))
        response = jsonify({'grid': grid})
        return response


step_fields_2d = api.model('Cellular_Automata_2_D', {
    'rule': fields.String(required=True, default='game_of_life',
                          description=f'Cellular Automata rule, available:{rules.keys()}'),
    'grid': fields.List(fields.List(fields.Integer()), required=True, description='grid to transform'),
})


@api.route('/CellularAutomata/2d/step')
class CellularAutomata2DStep(Resource):
    @api.doc(body=step_fields_2d)
    @api.expect(step_fields_2d, validate=True)
    def post(self):
        values = request.get_json()
        grid = values['grid']
        rule = values['rule']
        grid = RoundList([RoundList([value for value in row]) for row in grid])
        new_grid = update_grid_one_d(grid, rules[rule])

        response = jsonify({'grid': new_grid})
        return response


@api.route('/grid/1d/random')
class Grid1dRandom(Resource):
    @api.doc(params={'width': 'width', 'colors_count': 'colors count (states)'})
    def get(self):
        print(request.args)
        width = request.args.get('width')
        colors_count = request.args.get('colors_count')
        grid = RoundList(generate_random(
            input_list=[i for i in range(int(colors_count))],
            length=int(width)))
        response = jsonify({'grid': grid})
        return response


step_fields_1d = api.model('Cellular_Automata_1_D', {
    'wolfram_number': fields.Integer(required=True,
                                     description=f'number of rule',
                                     default=90),
    'neighborhood_size': fields.Integer(required=True,
                                        description='how many neighbor are checked',
                                        default=3),
    'colours': fields.List(fields.Integer(), required=True,
                           description=f'cells colors (states)',
                           default=[0, 1]),
    'grid': fields.List(fields.Integer(), required=True, description='grid to transform'),
})


@api.route('/CellularAutomata/1d/step')
class CellularAutomata1DStep(Resource):
    @api.doc(body=step_fields_1d)
    @api.expect(step_fields_1d, validate=True)
    def post(self):
        values = request.get_json()
        grid = values['grid']
        wolfram_number = values['wolfram_number']
        neighborhood_size = values['neighborhood_size']
        colours = values['colours']

        grid = RoundList(grid)
        rules = generate_rule(wolfram_number=int(wolfram_number),
                              neighborhood_size=int(neighborhood_size),
                              colours=colours)
        new_grid = cellular_automata_step_1d(input_list=grid, rules=rules)
        response = jsonify({'grid': new_grid})
        return response


@api.route('/test')
class ApiTest(Resource):
    @api.doc()
    def get(self):
        response = jsonify({
            "Message": "API running"
        })
        return response


if __name__ == '__main__':
    app.run(host="0.0.0.0", port=5000, debug=True)