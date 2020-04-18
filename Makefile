.PHONY: wasm webpack serve

wasm:
	wasm-pack build

webpack:
	cd www && npm run build

serve:
	cd www && npm run start
