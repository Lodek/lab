.PHONY: run
run: build_image
	docker run --rm -it zanzibar ghci example/runner.hs

.PHONY: build_image
build_image:
	docker build --tag zanzibar .
