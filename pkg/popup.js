import init, { run } from './snek_hq_extension.js';

async function bootstrap() {
    try {
        // Initialize WASM module
        await init();

        // Call the Rust export explicitly
        run();

        // Optional: Add any JS-side initialization
        console.log("Extension initialized successfully");
    } catch (error) {
        console.error("Initialization failed:", error);
    }
}

// Start the application
if (document.readyState === 'complete' || document.readyState === 'interactive') {
    bootstrap();
} else {
    document.addEventListener('DOMContentLoaded', bootstrap);
}