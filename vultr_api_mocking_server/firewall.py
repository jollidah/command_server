from flask import Blueprint, request, jsonify
import uuid

firewall_bp = Blueprint('firewall_bp', __name__)

# 메모리 내 방화벽 그룹 저장소
firewall_groups = {}
firewall_rules = {}

# List Firewall Groups
@firewall_bp.route('/v2/firewalls', methods=['GET'])
def list_firewall_groups():
    return jsonify({'firewall_groups': list(firewall_groups.values())}), 200

# Create Firewall Group
@firewall_bp.route('/v2/firewalls', methods=['POST'])
def create_firewall_group():
    print(request.json)
    data = request.json
    required_fields = ['description']
    if not all(field in data for field in required_fields):
        return jsonify({'error': 'Missing required fields'}), 400

    firewall_id = str(uuid.uuid4())
    firewall_group = {
        'id': firewall_id,
        'description': data['description'],
        'rules': []
    }
    firewall_groups[firewall_id] = firewall_group
    return jsonify({'firewall_group': firewall_group}), 200

# Get Firewall Group
@firewall_bp.route('/v2/firewalls/<id>', methods=['GET'])
def get_firewall_group(id):
    if id not in firewall_groups:
        return jsonify({'error': 'Firewall Group not found'}), 404
    return jsonify({'firewall_group': firewall_groups[id]}), 200

# Update Firewall Group
@firewall_bp.route('/v2/firewalls/<id>', methods=['PATCH'])
def update_firewall_group(id):
    if id not in firewall_groups:
        return jsonify({'error': 'Firewall Group not found'}), 404
    data = request.json
    if 'name' in data:
        firewall_groups[id]['name'] = data['name']
    return jsonify({'firewall_group': firewall_groups[id]}), 200

# Delete Firewall Group
@firewall_bp.route('/v2/firewalls/<id>', methods=['DELETE'])
def delete_firewall_group(id):
    if id not in firewall_groups:
        return jsonify({'error': 'Firewall Group not found'}), 404
    del firewall_groups[id]
    return jsonify({'message': 'Firewall Group deleted'}), 204

# List Firewall Rules
@firewall_bp.route('/v2/firewalls/<firewall_id>/rules', methods=['GET'])
def list_firewall_rules(firewall_id):
    if firewall_id not in firewall_groups:
        return jsonify({'error': 'Firewall Group not found'}), 404
    return jsonify({'rules': firewall_groups[firewall_id]['rules']}), 200

# Create Firewall Rule
@firewall_bp.route('/v2/firewalls/<firewall_id>/rules', methods=['POST'])
def create_firewall_rule(firewall_id):
    if firewall_id not in firewall_groups:
        return jsonify({'error': 'Firewall Group not found'}), 404
    data = request.json
    required_fields = ['action', 'protocol', 'port']
    if not all(field in data for field in required_fields):
        return jsonify({'error': 'Missing required fields'}), 400

    rule_id = str(uuid.uuid4())
    rule = {
        'id': rule_id,
        'action': data['action'],
        'protocol': data['protocol'],
        'port': data['port']
    }
    firewall_groups[firewall_id]['rules'].append(rule)
    return jsonify({'rule': rule}), 200

# Delete Firewall Rule
@firewall_bp.route('/v2/firewalls/<firewall_id>/rules/<rule_id>', methods=['DELETE'])
def delete_firewall_rule(firewall_id, rule_id):
    if firewall_id not in firewall_groups:
        return jsonify({'error': 'Firewall Group not found'}), 404
    rules = firewall_groups[firewall_id]['rules']
    rule = next((r for r in rules if r['id'] == rule_id), None)
    if not rule:
        return jsonify({'error': 'Firewall Rule not found'}), 404
    rules.remove(rule)
    return jsonify({'message': 'Firewall Rule deleted'}), 204

# Get Firewall Rule
@firewall_bp.route('/v2/firewalls/<firewall_id>/rules/<rule_id>', methods=['GET'])
def get_firewall_rule(firewall_id, rule_id):
    if firewall_id not in firewall_groups:
        return jsonify({'error': 'Firewall Group not found'}), 404
    rules = firewall_groups[firewall_id]['rules']
    rule = next((r for r in rules if r['id'] == rule_id), None)
    if not rule:
        return jsonify({'error': 'Firewall Rule not found'}), 404
    return jsonify({'rule': rule}), 200 