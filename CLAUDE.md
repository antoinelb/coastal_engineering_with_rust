# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based coastal engineering project that combines numerical methods, machine learning, and computer vision for coastal analysis. The project is currently in the planning phase with an 8-week implementation roadmap outlined in `study_plan.md`.

## Project Initialization

Since this is a new project without Rust infrastructure, first initialize with:
```bash
cargo init --name coastal_engineering
```

## Development Commands

Once the project is initialized, use these commands:

```bash
# Build the project
cargo build
cargo build --release

# Run tests
cargo test
cargo test -- --nocapture  # Show println! output during tests

# Run specific test
cargo test test_name

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings

# Run benchmarks (once added)
cargo bench

# Generate documentation
cargo doc --open
```

## Architecture Overview

The project will be structured around these core modules based on the study plan:

1. **Wave Analysis** (`src/waves/`)
   - NDBC data parsing and real-time analysis
   - Spectral analysis using FFT (rustfft crate)
   - Linear wave theory implementations

2. **Numerical Solvers** (`src/solvers/`)
   - 1D/2D shallow water equation solvers
   - Finite difference and finite volume methods
   - Parallel computation using rayon

3. **Sediment Transport** (`src/sediment/`)
   - CERC formula implementation
   - Kamphuis formula
   - Transport rate calculations

4. **Machine Learning** (`src/ml/`)
   - LSTM wave prediction models
   - Physics-informed neural networks (PINNs)
   - PyO3 integration for Python ML models

5. **Computer Vision** (`src/vision/`)
   - Shoreline detection algorithms
   - Satellite imagery processing (image crate)
   - Rip current detection

6. **API/Web Interface** (`src/api/`)
   - RESTful API using Actix-web
   - Real-time data endpoints

## Key Dependencies

Add these to `Cargo.toml` as the project develops:

```toml
[dependencies]
ndarray = "0.15"          # Numerical arrays
rayon = "1.7"             # Parallel computing
plotters = "0.3"          # Visualization
rustfft = "6.1"           # FFT operations
image = "0.24"            # Image processing
actix-web = "4"           # Web framework
pyo3 = "0.19"             # Python integration
serde = { version = "1.0", features = ["derive"] }  # Serialization
reqwest = "0.11"          # HTTP client for data fetching
tokio = { version = "1", features = ["full"] }      # Async runtime
```

## Data Sources

The project integrates with these coastal data sources:
- NDBC (National Data Buoy Center): https://www.ndbc.noaa.gov/data/realtime2/
- CDIP (Coastal Data Information Program): Spectral wave data
- Sentinel-2: Satellite imagery for shoreline detection
- HURDAT2: Hurricane track data

## Testing Strategy

- Unit tests for each numerical method implementation
- Integration tests for data parsing and API endpoints
- Validation against known analytical solutions
- Comparison with established tools (XBeach, CoastSat)

## Performance Considerations

- Use `ndarray` views to avoid unnecessary copying
- Leverage `rayon` for embarrassingly parallel operations
- Profile with `cargo flamegraph` for optimization
- Consider SIMD operations for numerical kernels

## External Tool Integration

- Python models are called via PyO3 bindings
- Consider creating Python wheel for reverse integration
- Maintain compatibility with common coastal engineering tools

## Code Style

- Follow Rust naming conventions (snake_case for functions/variables)
- Use descriptive variable names from coastal engineering domain
- Document physical units in comments (e.g., wave height in meters)
- Implement Display trait for domain types for debugging

## Claude Code Guidance

- Do not implement any code unless specifically asked for it
- Role is to act as a tutor and provide guidance
- Focus on explaining concepts, reviewing code, and offering strategic advice