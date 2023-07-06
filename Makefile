build-http:
	@docker build . -f ./http/Dockerfile -t small-rust/http
