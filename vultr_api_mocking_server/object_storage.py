from flask import Blueprint, request, jsonify
import uuid

object_storage_bp = Blueprint('object_storage_bp', __name__)

# 메모리 내 오브젝트 스토리지 저장소
object_storages = {}

# List Object Storages
@object_storage_bp.route('/v2/object-storages', methods=['GET'])
def list_object_storages():
    return jsonify({'object_storages': list(object_storages.values())}), 200

# Create Object Storage
@object_storage_bp.route('/v2/object-storages', methods=['POST'])
def create_object_storage():
    data = request.json
    required_fields = ['name', 'region']
    if not all(field in data for field in required_fields):
        return jsonify({'error': 'Missing required fields'}), 400

    storage_id = str(uuid.uuid4())
    object_storage = {
        'id': storage_id,
        'name': data['name'],
        'region': data['region'],
        'status': 'active'
    }
    object_storages[storage_id] = object_storage
    return jsonify({'object_storage': object_storage}), 200

# Get Object Storage
@object_storage_bp.route('/v2/object-storages/<id>', methods=['GET'])
def get_object_storage(id):
    if id not in object_storages:
        return jsonify({'error': 'Object Storage not found'}), 404
    return jsonify({'object_storage': object_storages[id]}), 200

# Delete Object Storage
@object_storage_bp.route('/v2/object-storages/<id>', methods=['DELETE'])
def delete_object_storage(id):
    if id not in object_storages:
        return jsonify({'error': 'Object Storage not found'}), 404
    del object_storages[id]
    return jsonify({'message': 'Object Storage deleted'}), 204

# Update Object Storage
@object_storage_bp.route('/v2/object-storages/<id>', methods=['PATCH'])
def update_object_storage(id):
    if id not in object_storages:
        return jsonify({'error': 'Object Storage not found'}), 404
    data = request.json
    if 'name' in data:
        object_storages[id]['name'] = data['name']
    if 'region' in data:
        object_storages[id]['region'] = data['region']
    return jsonify({'object_storage': object_storages[id]}), 200 