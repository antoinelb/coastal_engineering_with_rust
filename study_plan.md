# 2-Month Coastal Engineering Study Plan for Data Scientists
## 40 Hours Total | 5 Hours/Week | Rust + Numerical Methods + ML Applications

### Prerequisites & Setup (Before Week 1)
- Install Rust toolchain: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Set up VS Code with rust-analyzer extension
- Install Python environment with coastal libraries: `pip install coastlib pybeach oceanlyz xarray netcdf4`
- Create accounts for NDBC data access and GitHub

---

## Week 1: Foundations - Rust Basics & Wave Theory (5 hours)

### Learning Objectives
- Master Rust fundamentals for scientific computing
- Understand linear wave theory and dispersion relations
- Parse and analyze real wave data

### Daily Schedule
**Day 1-2 (2 hours)**: Rust Scientific Computing Basics
- **Study**: The Rust Book Chapters 1-5 (focus on ownership, borrowing, structs)
- **Resource**: https://doc.rust-lang.org/book/
- **Practice**: Install and explore `ndarray` crate
- **Exercise**: Create a simple matrix operations program using ndarray

**Day 3-4 (2 hours)**: Introduction to Linear Wave Theory
- **Study**: Sorensen "Basic Coastal Engineering" Chapters 2-3
- **Video**: IIT Madras NPTEL Lectures 1-3 on wave mechanics
- **Key Equations**: Master dispersion relation ω² = gk tanh(kh)
- **Exercise**: Derive wave properties (celerity, wavelength) by hand

**Day 5 (1 hour)**: Data Integration
- **Project**: Build a Rust program to parse NDBC buoy data
- **Resource**: https://www.ndbc.noaa.gov/data/realtime2/
- **Output**: Calculate significant wave height and peak period from raw data

### Week 1 Deliverable
Rust program that downloads and analyzes real-time wave data from NDBC buoys

---

## Week 2: Numerical Methods & Sediment Transport (5 hours)

### Learning Objectives
- Implement finite difference methods in Rust
- Understand sediment transport fundamentals
- Apply numerical schemes to coastal problems

### Daily Schedule
**Day 1-2 (2 hours)**: Finite Difference Methods
- **Study**: Finite difference theory for wave equations
- **Resource**: Bosboom & Stive online textbook (Chapters on numerical methods)
- **Practice**: Implement 1D wave equation solver in Rust
```rust
// Template structure
struct Wave1DSolver {
    eta: Array1<f64>,
    dx: f64, dt: f64, c: f64
}
```

**Day 3-4 (2 hours)**: Sediment Transport Basics
- **Study**: CERC formula and Kamphuis formula derivations
- **Resource**: Kamphuis textbook Chapters 4-5
- **Video**: USACE Coastal Engineering Manual sections
- **Exercise**: Calculate longshore transport rates using different formulas

**Day 5 (1 hour)**: Integration Exercise
- **Project**: Implement CERC formula in Rust
- **Data**: Use wave conditions from Week 1 NDBC data
- **Validation**: Compare with py-wave-runup Python results

### Week 2 Deliverable
Rust module for sediment transport calculations with validation against known solutions

---

## Week 3: Advanced Rust & Coastal Hydrodynamics (5 hours)

### Learning Objectives
- Master Rust parallelization with rayon
- Understand shallow water equations
- Build foundation for 2D modeling

### Daily Schedule
**Day 1-2 (2 hours)**: Parallel Computing in Rust
- **Study**: rayon crate documentation and examples
- **Resource**: Scientific Computing in Rust workshop videos
- **Practice**: Parallelize wave calculations from Week 1
- **Benchmark**: Compare performance vs serial implementation

**Day 3-4 (2 hours)**: Shallow Water Equations
- **Study**: Conservation laws and numerical stability
- **Resource**: Reeve, Chadwick & Fleming Chapter on hydrodynamics
- **Math**: Derive CFL condition for explicit schemes
- **Exercise**: Implement simple 1D shallow water solver

**Day 5 (1 hour)**: Visualization Setup
- **Tool**: Install and explore plotters crate
- **Project**: Add visualization to wave equation solver
- **Output**: Animated wave propagation plots

### Week 3 Deliverable
Parallel 1D shallow water solver with visualization capabilities

---

## Week 4: Machine Learning Applications - Wave Prediction (5 hours)

### Learning Objectives
- Apply ML to coastal engineering problems
- Build LSTM model for wave height prediction
- Interface Rust with Python ML models

### Daily Schedule
**Day 1-2 (2 hours)**: ML Fundamentals for Coastal Data
- **Study**: Time series analysis for wave data
- **Resource**: Fan et al. (2020) LSTM wave prediction paper
- **Tool**: Set up PyO3 for Python-Rust integration
- **Data**: Download 1-year NDBC historical data

**Day 3-4 (2 hours)**: LSTM Implementation
- **Framework**: Build LSTM model in Python (PyTorch)
- **Features**: Use wind speed, previous wave heights
- **Training**: 80/20 train/test split
- **Target**: 6-hour wave height prediction

**Day 5 (1 hour)**: Rust Integration
- **Project**: Create Rust wrapper for Python model
- **Performance**: Benchmark inference speed
- **API**: Design clean interface for predictions

### Week 4 Deliverable
Wave prediction system with Rust frontend and Python ML backend

---

## Week 5: Spectral Methods & Coastal Processes (5 hours)

### Learning Objectives
- Implement FFT-based spectral analysis
- Understand wave transformation processes
- Analyze directional wave spectra

### Daily Schedule
**Day 1-2 (2 hours)**: Spectral Analysis in Rust
- **Study**: Wave spectrum theory (JONSWAP, directional spreading)
- **Tool**: Implement FFT using rustfft crate
- **Resource**: spectrum-analyzer crate examples
- **Exercise**: Calculate wave spectra from time series

**Day 3-4 (2 hours)**: Wave Transformation
- **Theory**: Shoaling, refraction, diffraction
- **Math**: Implement Snell's law for wave refraction
- **Resource**: CEM equations for wave transformation
- **Project**: Build refraction calculator

**Day 5 (1 hour)**: Practical Application
- **Data**: CDIP spectral wave data
- **Analysis**: Compare measured vs theoretical spectra
- **Visualization**: Plot directional wave roses

### Week 5 Deliverable
Spectral wave analysis tool with directional capabilities

---

## Week 6: Finite Volume Methods & Storm Surge (5 hours)

### Learning Objectives
- Implement 2D finite volume solver basics
- Understand storm surge mechanics
- Apply ML to surge prediction

### Daily Schedule
**Day 1-2 (2 hours)**: Finite Volume Fundamentals
- **Theory**: Godunov-type methods, Riemann solvers
- **Resource**: XBeach source code structure study
- **Implementation**: Basic 2D grid structure in Rust
```rust
struct ShallowWater2D {
    h: Array2<f64>,
    u: Array2<f64>, 
    v: Array2<f64>
}
```

**Day 3-4 (2 hours)**: Storm Surge Modeling
- **Study**: Wind stress and pressure effects
- **Data**: Download Hurricane track data (HURDAT2)
- **Physics**: Implement simplified surge equations
- **Validation**: Compare with historical events

**Day 5 (1 hour)**: ML Enhancement
- **Concept**: Physics-Informed Neural Networks (PINNs)
- **Resource**: Recent PINN papers for storm surge
- **Project**: Design hybrid surge predictor architecture

### Week 6 Deliverable
Prototype 2D storm surge calculator with ML bias correction framework

---

## Week 7: Computer Vision for Coastal Monitoring (5 hours)

### Learning Objectives
- Apply computer vision to coastal problems
- Implement shoreline detection algorithms
- Build practical monitoring tools

### Daily Schedule
**Day 1-2 (2 hours)**: Image Processing Basics
- **Tool**: Explore image crate for Rust
- **Study**: U-Net architecture for segmentation
- **Resource**: CoastSat methodology papers
- **Data**: Download Sentinel-2 coastal imagery

**Day 3-4 (2 hours)**: Shoreline Detection
- **Algorithm**: Edge detection and morphological operations
- **Implementation**: Basic shoreline extractor in Rust
- **Validation**: Compare with CoastSat Python results
- **Enhancement**: Sub-pixel refinement techniques

**Day 5 (1 hour)**: Rip Current Detection
- **Study**: YOLO-Rip model architecture
- **Data**: Rip current image dataset
- **Project**: Design detection pipeline architecture

### Week 7 Deliverable
Shoreline change detection tool processing satellite imagery

---

## Week 8: Integration & Final Project (5 hours)

### Learning Objectives
- Integrate all learned components
- Build comprehensive coastal analysis system
- Prepare for real-world applications

### Daily Schedule
**Day 1-2 (2 hours)**: System Architecture
- **Design**: Modular coastal engineering toolkit in Rust
- **Components**: Wave analysis, sediment transport, ML predictions
- **API**: RESTful interface using Actix-web
- **Database**: Time series storage design

**Day 3-4 (2 hours)**: Final Project Implementation
- **Choose one**:
  1. Real-time coastal conditions dashboard
  2. Beach erosion prediction system
  3. Wave energy assessment tool
- **Requirements**: Use 3+ techniques learned
- **Data**: Integrate multiple data sources

**Day 5 (1 hour)**: Documentation & Next Steps
- **Document**: Code with examples
- **Benchmark**: Performance comparisons
- **Plan**: Identify areas for deeper study
- **Community**: Share on GitHub, join coastal-list

### Week 8 Deliverable
Complete coastal engineering application showcasing Rust, numerical methods, and ML

---

## Key Resources Summary

### Books (Priority Order)
1. Sorensen - "Basic Coastal Engineering" ($200)
2. Kamphuis - "Introduction to Coastal Engineering" ($150)
3. Bosboom & Stive - "Coastal Dynamics" (Free online)

### Online Courses
1. IIT Madras NPTEL Coastal Engineering (Free)
2. TU Delft Building with Nature (Free audit)

### Software & Tools
1. Rust: ndarray, rayon, plotters, rustfft
2. Python: coastlib, pybeach, xarray
3. Data: NDBC, CDIP, CoastSat

### Communities
1. Scientific Computing in Rust (https://scientificcomputing.rs)
2. Coastal List mailing list
3. GitHub: awesome-coastal repository

### Study Tips
- Use Claude for code reviews and debugging
- Join rust-scicomp Zulip chat for help
- Implement small examples for each concept
- Validate against published results
- Focus on one integration at a time

### Assessment Milestones
- Week 2: Functional wave data analyzer
- Week 4: ML prediction with >0.9 R²
- Week 6: 2D numerical solver running
- Week 8: Integrated application deployed

This plan provides a structured path from fundamentals to practical applications, leveraging your data science background while building expertise in coastal engineering and Rust programming.