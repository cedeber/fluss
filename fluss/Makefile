dev : src/lib.rs
	wasm-pack build --dev --target web --out-dir ../app/public/pkg/

release : src/lib.rs
	wasm-pack build --target web --out-dir public/pkg
	wasm-opt -Os -o public/pkg/fluss.wasm public/pkg/fluss_bg.wasm
	terser public/pkg/fluss.js --output public/pkg/fluss.min.js --compress --module --ecma 7
	rm public/pkg/fluss_bg.wasm
	rm public/pkg/fluss.js
	mv public/pkg/fluss.wasm public/pkg/fluss_bg.wasm
	mv public/pkg/fluss.min.js public/pkg/fluss.js

clean :
	rm -rf fluss/target/
	rm -rf ../app/public/pkg/