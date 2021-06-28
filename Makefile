# Init images
init-image-1.0:
	docker build --no-cache -t becavalier/kubeless:v1 -f Dockerfile.1.0.init .

# Runtime images
runtime-image-1.0:
	docker build --no-cache -f Dockerfile -t becavalier/kubeless:rt-v1 .
