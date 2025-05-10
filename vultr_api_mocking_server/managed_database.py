from flask import Blueprint, request, jsonify
import uuid

managed_database_bp = Blueprint('managed_database_bp', __name__)

# 메모리 내 관리형 데이터베이스 저장소
managed_databases = {}

# List Managed Databases
@managed_database_bp.route('/v2/managed-databases', methods=['GET'])
def list_managed_databases():
    return jsonify({'managed_databases': list(managed_databases.values())}), 200

# Create Managed Database
@managed_database_bp.route('/v2/managed-databases', methods=['POST'])
def create_managed_database():
    data = request.json
    required_fields = ['name', 'engine', 'version']
    if not all(field in data for field in required_fields):
        return jsonify({'error': 'Missing required fields'}), 400

    db_id = str(uuid.uuid4())
    managed_database = {
        'id': db_id,
        'name': data['name'],
        'engine': data['engine'],
        'version': data['version'],
        'status': 'active'
    }
    managed_databases[db_id] = managed_database
    return jsonify({'managed_database': managed_database}), 200

# Get Managed Database
@managed_database_bp.route('/v2/managed-databases/<id>', methods=['GET'])
def get_managed_database(id):
    if id not in managed_databases:
        return jsonify({'error': 'Managed Database not found'}), 404
    return jsonify({'managed_database': managed_databases[id]}), 200

# Update Managed Database
@managed_database_bp.route('/v2/managed-databases/<id>', methods=['PATCH'])
def update_managed_database(id):
    if id not in managed_databases:
        return jsonify({'error': 'Managed Database not found'}), 404
    data = request.json
    if 'name' in data:
        managed_databases[id]['name'] = data['name']
    if 'engine' in data:
        managed_databases[id]['engine'] = data['engine']
    if 'version' in data:
        managed_databases[id]['version'] = data['version']
    return jsonify({'managed_database': managed_databases[id]}), 200

# Delete Managed Database
@managed_database_bp.route('/v2/managed-databases/<id>', methods=['DELETE'])
def delete_managed_database(id):
    if id not in managed_databases:
        return jsonify({'error': 'Managed Database not found'}), 404
    del managed_databases[id]
    return jsonify({'message': 'Managed Database deleted'}), 204 