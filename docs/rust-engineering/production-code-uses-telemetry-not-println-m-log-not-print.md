<!-- Section from Microsoft Pragmatic Rust Guidelines -->
# Production Code Uses Telemetry Not Println M Log Not Print

## Production code uses telemetry, not println (M-LOG-NOT-PRINT) { #M-LOG-NOT-PRINT }

<why>diagnostics available where they are needed.</why>

Production code paths should emit diagnostics through the project's telemetry framework rather than via `println!` or `dbg!`. Console output is reserved for CLIs that intentionally write to stdout as their user interface.



