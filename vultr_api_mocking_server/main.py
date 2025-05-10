from flask import Flask
from instance import instance_bp

app = Flask(__name__)
app.register_blueprint(instance_bp)

# 메모리 내 인스턴스 저장소
instances = {}

if __name__ == '__main__':
    app.run(port=3000, debug=True)
