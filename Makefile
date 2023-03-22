.PHONY: deploy
deploy:
	@echo "====> deploying to github"
	rm -rf /tmp/build
	git worktree add /tmp/build/ gh-pages
	wasm-pack build --target web
	rm -rf /tmp/build/*
	mkdir /tmp/build/pkg
	cp -rp pkg/* /tmp/build/pkg/
	rm -rf /tmp/build/pkg/.gitignore
	cp index.html /tmp/build/
	cp index.css /tmp/build
	cd /tmp/build/ && \
		git update-ref -d refs/heads/gh-pages && \
		git add -A && \
		git commit -m "deployed on $(shell date) by ${USER}" && \
		git push --force origin gh-pages