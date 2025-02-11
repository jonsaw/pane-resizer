default: serve

serve:
	trunk serve

install:
	pnpm install

jspackages-install:
	pnpm install --prefix ./jspackages

jspackages-build:
	pnpm run --prefix ./jspackages build

tailwindcss-build:
	npx tailwindcss -i ./pane-resizer-docs/styles.scss -o ./diststyles/styles.css

tailwindcss-watch:
	npx tailwindcss -i ./pane-resizer-docs/styles.scss -o ./diststyles/styles.css --watch
