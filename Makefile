default: serve

serve:
	trunk serve

jspackages-install:
	pnpm install --prefix ./jspackages

jspackages-build:
	pnpm run --prefix ./jspackages build

tailwindcss-build:
	npm exec --prefix ./jspackages tailwindcss -- -c ./tailwind.config.js -i ./styles.scss -o ./diststyles/styles.css

tailwindcss-watch:
	npm exec --prefix ./jspackages tailwindcss -- -c ./tailwind.config.js -i ./styles.scss -o ./diststyles/styles.css --watch
