from flask import Blueprint, request, jsonify
import uuid

instance_bp = Blueprint('instance_bp', __name__)

# 메모리 내 인스턴스 저장소
instances = {}

# Create Instance (필수 필드만)
@instance_bp.route('/v2/instances', methods=['POST'])
def create_instance():
    data = request.json
    required_fields = ['region', 'plan', 'os_id']
    if not all(field in data for field in required_fields):
        return jsonify({'error': 'Missing required fields'}), 400

    instance_id = str(uuid.uuid4())
    instance = {
        'id': instance_id,
        'region': data['region'],
        'plan': data['plan'],
        'os_id': data['os_id'],
        'label': 'mock-instance',
        'status': 'active',
        'vpc_ids': []
    }
    instances[instance_id] = instance
    return jsonify({'instance': instance}), 200

# List Instances
@instance_bp.route('/v2/instances', methods=['GET'])
def list_instances():
    return jsonify({'instances': list(instances.values())}), 200

# Get Instance
@instance_bp.route('/v2/instances/<id>', methods=['GET'])
def get_instance(id):
    if id not in instances:
        return jsonify({'error': 'Instance not found'}), 404
    return jsonify({'instance': instances[id]}), 200

# Update Instance (label only)
@instance_bp.route('/v2/instances/<id>', methods=['PATCH'])
def update_instance(id):
    if id not in instances:
        return jsonify({'error': 'Instance not found'}), 404
    data = request.json
    if 'label' in data:
        instances[id]['label'] = data['label']
    return jsonify({'instance': instances[id]}), 200

# Attach VPC
@instance_bp.route('/v2/instances/<id>/vpcs', methods=['POST'])
def attach_vpc(id):
    if id not in instances:
        return jsonify({'error': 'Instance not found'}), 404
    data = request.json
    vpc_id = data.get('vpc_id')
    if not vpc_id:
        return jsonify({'error': 'vpc_id is required'}), 400
    instances[id]['vpc_ids'].append(vpc_id)
    return jsonify({'instance': instances[id]}), 200

# Detach VPC
@instance_bp.route('/v2/instances/<id>/vpcs/<vpc_id>', methods=['DELETE'])
def detach_vpc(id, vpc_id):
    if id not in instances:
        return jsonify({'error': 'Instance not found'}), 404
    instances[id]['vpc_ids'] = [v for v in instances[id]['vpc_ids'] if v != vpc_id]
    return jsonify({'instance': instances[id]}), 200
