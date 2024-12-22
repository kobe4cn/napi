build-js:
	@echo "Building JS"
	@cd napi-example && yarn build

run-js:
	@echo "Running JS"
	@cd napi-example && node
