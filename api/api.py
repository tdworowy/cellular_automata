from flask import Flask, request, jsonify
from flask_restx import Api, fields, Resource

from _1D.cellular_automata import generate_random, generate_rule, cellular_automata_step_1d
from _2D.general_2d_automata import generate_grid_random_cells, generate_grid_central, update_grid_two_d, rules
import numpy as np
import random

from _2D.langton_ant import generate_grid_ant, update_grid_ant

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
        response = jsonify({'grid': grid.tolist()})
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
        response = jsonify({'grid': grid.tolist()})
        return response


@api.route('/grid/2d/ant')
class Grid2dRandom(Resource):
    @api.doc(params={'width': 'width', 'height': 'height', 'ant_count': 'number of ants', 'random_init_turn': 'random '
                                                                                                              'initial turn of ant'})
    def get(self):
        print(request.args)
        width = request.args.get('width')
        height = request.args.get('height')
        ant_count = request.args.get('ant_count')
        random_init_turn = request.args.get('random_init_turn')
        grid, turns = generate_grid_ant(width=int(width),
                                        height=int(height),
                                        ant_count=int(ant_count),
                                        random_init_turn=int(random_init_turn)
                                        )
        response = jsonify({'grid': grid.tolist(), 'turns': turns})
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
        grid = np.array(grid)
        new_grid = update_grid_two_d(grid, rules[rule])

        response = jsonify({'grid': new_grid.tolist()})
        return response


step_fields_ant = api.model('Langton ant', {
    'grid': fields.List(fields.List(fields.Raw), required=True, description='grid to transform'),
    'turns': fields.Raw(required=True, description='Ant turns')
})


@api.route('/CellularAutomata/2d/ant')
class CellularAutomata2DAntStep(Resource):
    @api.doc(body=step_fields_ant)
    @api.expect(step_fields_ant, validate=True)
    def post(self):
        values = request.get_json()
        grid = values['grid']
        turns = values['turns']
        grid = np.array(grid)
        new_grid, turns = update_grid_ant(grid, turns)

        response = jsonify({'grid': new_grid.tolist(), 'turns': turns})
        return response


@api.route('/grid/1d/random')
class Grid1dRandom(Resource):
    @api.doc(params={'width': 'width', 'colors_count': 'colors count (states)'})
    def get(self):
        print(request.args)
        width = request.args.get('width')
        colors_count = request.args.get('colors_count')
        grid = generate_random(
            input_list=tuple(i for i in range(int(colors_count))),
            length=int(width))
        response = jsonify({'grid': grid.tolist()})
        return response


@api.route('/grid/1d/center')
class Grid1dCenter(Resource):
    @api.doc(params={'width': 'width', 'colors_count': 'colors count (states)'})
    def get(self):
        print(request.args)
        width = request.args.get('width')
        colors_count = request.args.get('colors_count')
        grid = np.full((int(width), 1), 0)
        grid[len(grid) // 2] = random.randrange(1, int(colors_count))

        response = jsonify({'grid': grid.tolist()})
        return response


step_fields_1d = api.model('Cellular_Automata_1_D', {
    'wolfram_number': fields.String(required=True,
                                    description=f'number of rule',
                                    default='90'),
    'neighborhood_size': fields.String(required=True,
                                       description='how many neighbor are checked',
                                       default='3'),
    'colours': fields.String(required=True,
                             description=f'cells colors (states)',
                             default='0,1'),
    'grid': fields.List(fields.List(fields.Integer()), required=True, description='grid to transform'),
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

        grid = np.array(grid)
        rules = generate_rule(wolfram_number=int(wolfram_number),
                              neighborhood_size=int(neighborhood_size),
                              colours=[int(color) for color in colours.split(',')])
        new_grid = cellular_automata_step_1d(input_list=grid, rules=rules)
        response = jsonify({'grid': new_grid.tolist()})
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
