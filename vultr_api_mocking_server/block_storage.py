from flask import Blueprint, request, jsonify
import uuid

block_storage_bp = Blueprint('block_storage_bp', __name__)

# 메모리 내 블록 스토리지 저장소
block_storages = {}

# Get Block Storage
@block_storage_bp.route('/v2/block-storages/<id>', methods=['GET'])
def get_block_storage(id):
    if id not in block_storages:
        return jsonify({'error': 'Block Storage not found'}), 404
    return jsonify({'block_storage': block_storages[id]}), 200

# List Block Storage
@block_storage_bp.route('/v2/block-storages', methods=['GET'])
def list_block_storages():
    return jsonify({'block_storages': list(block_storages.values())}), 200

# Delete Block Storage
@block_storage_bp.route('/v2/block-storages/<id>', methods=['DELETE'])
def delete_block_storage(id):
    if id not in block_storages:
        return jsonify({'error': 'Block Storage not found'}), 404
    del block_storages[id]
    return jsonify({'message': 'Block Storage deleted'}), 204

# Update Block Storage
@block_storage_bp.route('/v2/block-storages/<id>', methods=['PATCH'])
def update_block_storage(id):
    if id not in block_storages:
        return jsonify({'error': 'Block Storage not found'}), 404
    data = request.json
    if 'label' in data:
        block_storages[id]['label'] = data['label']
    return jsonify({'block_storage': block_storages[id]}), 200

# Attach Block Storage
@block_storage_bp.route('/v2/block-storages/<id>/attach', methods=['POST'])
def attach_block_storage(id):
    if id not in block_storages:
        return jsonify({'error': 'Block Storage not found'}), 404
    data = request.json
    instance_id = data.get('instance_id')
    if not instance_id:
        return jsonify({'error': 'instance_id is required'}), 400
    block_storages[id]['attached_instance'] = instance_id
    return jsonify({'block_storage': block_storages[id]}), 200

# Detach Block Storage
@block_storage_bp.route('/v2/block-storages/<id>/detach', methods=['POST'])
def detach_block_storage(id):
    if id not in block_storages:
        return jsonify({'error': 'Block Storage not found'}), 404
    block_storages[id].pop('attached_instance', None)
    return jsonify({'block_storage': block_storages[id]}), 200
