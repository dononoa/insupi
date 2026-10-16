import init, { grep_search } from './pkg/wasm_grep.js';

let isWasmLoaded = false;

async function initWasm() {
    await init();
    isWasmLoaded = true;
    self.postMessage({ type: 'READY' });
}
initWasm();

self.onmessage = async (e) => {
    const { action, text, pattern, ignoreCase } = e.data;

    if (action === 'SEARCH') {
        if (!isWasmLoaded) {
            self.postMessage({ type: 'ERROR', error: 'Wasm is not loaded yet.' });
            return;
        }

        try {
            const startTime = performance.now();
            const results = grep_search(text, pattern, ignoreCase);
            const endTime = performance.now();

            self.postMessage({
                type: 'RESULTS',
                results: results,
                timeMs: (endTime - startTime).toFixed(2),
                count: results.length
            });
        } catch (err) {
            self.postMessage({ type: 'ERROR', error: err.toString() });
        }
    }
};
