# 2-Month Numerical Coastal Engineering Study Plan with Rust Implementation

*Note: Extended to 9 weeks to comprehensively cover coastal protection structures alongside core topics*

## Study plan overview for rapid skill development

This comprehensive 9-week plan allocates 5 hours/week (45 total hours) to develop practical skills in numerical coastal engineering with Rust, inspired by Marine Labs' real-time coastal intelligence approach. The curriculum emphasizes hands-on implementation while building from fundamental concepts to real-world applications, including design and analysis of coastal protection structures.

## Week 1-2: Foundation and mathematical refreshers

### Week 1: Coastal Engineering Fundamentals & Rust Setup (5 hours)

**Learning Objectives:**
- Understand Marine Labs' approach to real-time coastal monitoring
- Review essential wave mechanics and linear wave theory
- Set up Rust development environment for scientific computing

**Mathematical Review (1.5 hours):**
- Linear wave theory basics: dispersion relation ω² = gk tanh(kh)
- Wave parameters: height (H), period (T), wavelength (L), water depth (h)
- Review PDEs: wave equation ∂²u/∂t² = c²∇²u
- Finite difference basics for spatial discretization

**Rust Foundation (2 hours):**
- Install Rust toolchain and VS Code with rust-analyzer
- Create first scientific computing project structure
- Essential crates setup:
```toml
[dependencies]
ndarray = "0.15"        # NumPy-like arrays
peroxide = "0.34"       # Scientific computing
plotters = "0.3"        # Visualization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

**Hands-on Project (1.5 hours):**
Build a **Wave Parameter Calculator** CLI tool that:
- Calculates wavelength from period using dispersion relation
- Computes wave celerity and group velocity
- Implements shallow/deep water approximations
- Outputs results in JSON format

### Week 2: Data Structures & Coastal Protection Basics (5 hours)

**Learning Objectives:**
- Master Rust's ndarray for scientific data
- Understand fundamental coastal protection structures
- Implement wave transmission and reflection calculations

**Mathematical Review (1 hour):**
- Wave transmission coefficient: Kt = Ht/Hi
- Wave reflection coefficient: Kr = Hr/Hi
- Breakwater types: rubble mound, vertical wall, floating
- Hudson formula for armour stability: W = (ρr H³)/(KD(Sr-1)³cotα)

**Rust Implementation (2.5 hours):**
```rust
use ndarray::{Array1, Array2};
use std::f64::consts::PI;

struct WaveTimeSeries {
    timestamps: Array1<f64>,
    elevation: Array1<f64>,
    sample_rate: f64,
}

struct Breakwater {
    type_: BreakwaterType,
    crest_height: f64,
    crest_width: f64,
    porosity: f64,
}

impl Breakwater {
    fn transmission_coefficient(&self, wave_height: f64, period: f64, depth: f64) -> f64 {
        // Van der Meer formulas for transmission
        match self.type_ {
            BreakwaterType::RubbleMound => {
                let freeboard = self.crest_height - depth;
                let relative_fb = freeboard / wave_height;
                (-0.4 * relative_fb + 0.64).max(0.0).min(1.0)
            }
            BreakwaterType::Floating => {
                // Implement floating breakwater transmission
            }
            _ => 0.1 // Simplified for vertical walls
        }
    }
}
```

**Breakwater Analysis Project (1.5 hours):**
Create a **Breakwater Performance Calculator** that:
- Implements transmission/reflection coefficients
- Calculates required armour stone weight (Hudson formula)
- Compares different breakwater configurations
- Outputs performance metrics for various wave conditions

## Week 3-4: Numerical methods and real-time processing

### Week 3: PDE Solvers & Wave-Structure Interaction (5 hours)

**Learning Objectives:**
- Implement finite difference method for wave equation
- Model wave diffraction around structures
- Simulate wave transformation over coastal defences

**Mathematical Review (1.5 hours):**
- Mild-slope equation for wave propagation: ∇·(CCg∇φ) + k²CCgφ = 0
- Sommerfeld radiation condition for open boundaries
- Wave diffraction patterns around breakwaters
- Energy dissipation over porous structures

**Rust Implementation (2.5 hours):**
```rust
use ndarray::Array2;
use rayon::prelude::*;

struct CoastalDomain {
    bathymetry: Array2<f64>,
    structures: Vec<Structure>,
    dx: f64,
    dt: f64,
}

struct Structure {
    geometry: Array2<bool>,  // Structure mask
    porosity: f64,
    friction_coefficient: f64,
}

impl CoastalDomain {
    fn solve_mild_slope(&mut self, incident_wave: &Wave) -> Array2<Complex<f64>> {
        // Implement mild-slope equation solver
        // Include structure-induced dissipation
    }
    
    fn apply_structure_boundary(&mut self, structure: &Structure) {
        // Handle partial reflection/transmission
    }
}
```

**Wave-Structure Project (1 hour):**
Build a **Harbour Wave Model** that:
- Solves mild-slope equation with breakwater
- Visualizes wave height distribution in harbour
- Shows diffraction patterns behind breakwater
- Calculates wave height reduction effectiveness

### Week 4: Real-time Data Streaming (5 hours)

**Learning Objectives:**
- Master async Rust with tokio
- Implement real-time data processing pipeline
- Build event-driven architecture

**Stream Processing Focus (2 hours):**
- Study SeaStreamer for backend-agnostic streaming
- Implement rolling statistics with time windows
- Handle out-of-order and missing data

**Rust Async Implementation (2 hours):**
```rust
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);
    
    // Sensor data producer
    tokio::spawn(async move {
        loop {
            let data = fetch_sensor_data().await;
            tx.send(data).await.unwrap();
            tokio::time::sleep(Duration::from_secs(360)).await;
        }
    });
    
    // Real-time processor
    while let Some(data) = rx.recv().await {
        process_wave_data(data).await;
    }
}
```

**Real-time Dashboard Project (1 hour):**
Create a **Live Wave Monitor** that:
- Connects to NOAA buoy API every 6 minutes
- Calculates running statistics over 24-hour window
- Triggers alerts for threshold exceedances
- Outputs to web dashboard using warp/axum

## Week 5-7: Coastal protection, machine learning, and adaptation

### Week 5: Coastal Protection Design & Analysis (5 hours)

**Learning Objectives:**
- Design rubble mound breakwaters using empirical formulas
- Analyze coastal protection system effectiveness
- Implement optimization for structure placement

**Design Formulas Review (1.5 hours):**
- Van der Meer formulas for rock stability
- Overtopping discharge: q = 0.067/√tanα × γb × ξ × exp(-4.75 Rc/Hs)
- Toe protection and scour calculations
- Filter layer design (D15/d85 criteria)

**Protection System Implementation (2.5 hours):**
```rust
use optimization::Optimizer;

struct CoastalProtectionDesigner {
    site_conditions: SiteData,
    design_criteria: DesignCriteria,
}

struct RubbleMoundDesign {
    armour_weight: f64,
    armour_thickness: f64,
    filter_layers: Vec<LayerSpec>,
    crest_elevation: f64,
    toe_protection: ToeDesign,
}

impl CoastalProtectionDesigner {
    fn design_breakwater(&self, target_transmission: f64) -> RubbleMoundDesign {
        // Iterative design process
        let design_wave = self.calculate_design_wave();
        let armour_weight = self.hudson_formula(design_wave);
        let crest_height = self.optimize_crest_height(target_transmission);
        
        RubbleMoundDesign {
            armour_weight,
            armour_thickness: 2.0 * (armour_weight / self.rock_density).powf(1.0/3.0),
            filter_layers: self.design_filter_layers(),
            crest_elevation: crest_height,
            toe_protection: self.design_toe_protection(),
        }
    }
    
    fn evaluate_overtopping(&self, design: &RubbleMoundDesign) -> f64 {
        // EurOtop overtopping formulas
    }
}
```

**Coastal Defence Project (1 hour):**
Build a **Breakwater Design Tool** that:
- Takes site wave conditions and bathymetry
- Designs optimal rubble mound cross-section
- Evaluates multiple coastal protection options (groins, seawalls, detached breakwaters)
- Performs cost-benefit analysis
- Generates construction drawings with dimensions

### Week 6: Statistical Forecasting & ML Applications (5 hours)

**Learning Objectives:**
- Implement time series forecasting for waves
- Apply machine learning to coastal data
- Understand ensemble methods

**Mathematical Review (1 hour):**
- ARIMA models for wave height prediction
- Extreme value theory (GEV distribution)
- Neural network basics for time series

**ML Implementation with Candle (2.5 hours):**
```rust
use candle_core::{Device, Tensor};
use candle_nn::{Module, VarBuilder};

struct WavePredictor {
    lstm: candle_nn::LSTM,
    device: Device,
}

impl WavePredictor {
    fn forecast(&self, historical: &Tensor) -> Result<Tensor> {
        // LSTM-based wave height prediction
        let hidden = self.lstm.forward(historical)?;
        // Return next 6-hour forecast
    }
}
```

**Forecasting Project (1.5 hours):**
Build a **Wave Height Predictor** that:
- Uses augurs crate for ARIMA baseline
- Implements simple LSTM with candle
- Compares statistical vs ML approaches
- Provides uncertainty estimates

### Week 7: Climate Adaptation & Nature-Based Solutions (5 hours)

**Learning Objectives:**
- Implement sea level rise impact on structures
- Design hybrid grey-green infrastructure
- Create adaptive management tools

**Climate & Nature-Based Solutions (2 hours):**
- Living shorelines and oyster reefs
- Beach nourishment calculations
- Dune erosion models (SBEACH approach)
- Structure performance under SLR scenarios

**Adaptive Design Implementation (2 hours):**
```rust
struct AdaptiveCoastalDefence {
    initial_design: CoastalStructure,
    slr_scenarios: Vec<SLRScenario>,
    adaptation_triggers: Vec<Trigger>,
}

impl AdaptiveCoastalDefence {
    fn evaluate_performance_trajectory(&self) -> Vec<PerformanceMetric> {
        // Assess structure performance over time
    }
    
    fn design_living_shoreline(&self, site: &SiteConditions) -> HybridDesign {
        // Combine hard structures with nature-based features
        HybridDesign {
            oyster_reef: self.size_oyster_reef(site),
            marsh_platform: self.design_marsh_elevation(site),
            sill_structure: self.design_low_sill(site),
        }
    }
    
    fn optimize_beach_nourishment(&self) -> NourishmentPlan {
        // Dean's equilibrium profile approach
        let volume = self.calculate_fill_volume();
        let grain_size = self.match_native_sediment();
        NourishmentPlan { volume, grain_size, placement_strategy }
    }
}
```

**Resilience Dashboard Project (1 hour):**
Create a **Coastal Resilience Planner** that:
- Evaluates multiple protection strategies
- Includes nature-based solutions
- Projects performance under climate change
- Optimizes for co-benefits (habitat, recreation)
- Generates adaptive management timeline

## Week 8-9: Port protection and system integration

### Week 8: Port Protection & Operations (5 hours)

**Learning Objectives:**
- Design port entrance breakwaters
- Optimize harbour tranquility
- Integrate protection with operations

**Port Protection Focus (2 hours):**
- Harbour resonance and seiching
- Breakwater gap optimization
- Wave penetration coefficients
- Moored vessel response

**Protected Port Implementation (2 hours):**
```rust
struct PortProtectionSystem {
    layout: HarbourLayout,
    breakwaters: Vec<Breakwater>,
    berths: Vec<Berth>,
}

impl PortProtectionSystem {
    fn analyze_wave_penetration(&self, incident_waves: &WaveSpectrum) -> Array2<f64> {
        // Boussinesq or mild-slope model for harbour
    }
    
    fn optimize_entrance_width(&self) -> f64 {
        // Balance wave protection vs navigation
        let wave_reduction = |width| self.calculate_transmission(width);
        let navigation_safety = |width| self.vessel_passage_risk(width);
        
        // Multi-objective optimization
        optimize::minimize(|w| {
            wave_reduction(w) + navigation_safety(w)
        })
    }
    
    fn calculate_berth_availability(&self, forecast: &Forecast) -> Vec<BerthStatus> {
        // Account for breakwater effectiveness
        let protected_conditions = self.apply_protection(forecast);
        self.berths.iter().map(|berth| {
            berth.check_operational_limits(&protected_conditions)
        }).collect()
    }
}
```

**Port Design Project (1 hour):**
Build a **Protected Harbour Designer** that:
- Layouts optimal breakwater configuration
- Minimizes wave heights at berths
- Calculates construction costs
- Simulates vessel movements in protected basin
- Provides operational improvement metrics

### Week 9: Complete System Integration (5 hours)

**Learning Objectives:**
- Integrate all components into production system
- Deploy to cloud infrastructure
- Create comprehensive documentation

**System Architecture (1.5 hours):**
- Design microservices architecture
- Implement message queuing with Redis
- Set up PostgreSQL with TimescaleDB
- Configure Docker containers

**Final Integration Project (3.5 hours):**
Build a **Coastal Intelligence & Protection Platform** that combines:
- Real-time data ingestion from multiple sources
- Wave propagation modeling with structure interaction
- Breakwater and coastal defence design tools
- ML-based forecasting with uncertainty quantification
- Port protection effectiveness monitoring
- Operational decision support for protected harbours
- Climate adaptation planning for coastal structures
- Living shoreline performance tracking
- Web-based visualization dashboard with structure overlays

**Platform Features:**
- Interactive breakwater design module
- Real-time structure performance monitoring
- Wave reduction effectiveness tracking
- Overtopping alert system
- Maintenance scheduling based on storm damage
- Cost-benefit analysis tools
- Environmental impact assessment

**Deployment Checklist:**
- Containerize with Docker
- Set up CI/CD pipeline
- Configure monitoring and alerts
- Write comprehensive documentation
- Create user guide and API docs

## Essential Rust libraries for the journey

```toml
[dependencies]
# Core Scientific Computing
ndarray = "0.15"
ndarray-linalg = "0.16" 
peroxide = "0.34"
nalgebra = "0.33"

# Oceanographic
gsw = "0.2"  # Seawater properties
netcdf = "0.8"  # Data format

# Time Series & ML
augurs = "0.3"
candle-core = "0.6"
candle-nn = "0.6"

# Real-time Processing
tokio = { version = "1.40", features = ["full"] }
sea-streamer = "0.5"
tokio-tungstenite = "0.24"

# Visualization
plotters = "0.3"
plotly = "0.13"

# Web & API
axum = "0.7"
reqwest = "0.12"
tower = "0.5"

# Data Processing
polars = "0.44"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Ongoing learning resources

**Daily Practice:**
- Implement one numerical method each week
- Study USACE Coastal Engineering Manual sections
- Read papers from Coastal Engineering journal
- Practice with CEDAS/CRESS design tools for validation

**Key References for Coastal Protection:**
- "The Rock Manual" (CIRIA/CUR/CETMEF)
- EurOtop Overtopping Manual
- Coastal Engineering Manual (USACE)
- Van der Meer's publications on breakwater design

**Community Engagement:**
- Join Rust Scientific Computing Zulip chat
- Participate in COPRI Coastal Structures conferences
- Follow Marine Labs blog for real-world applications
- Share progress on GitHub

**Advanced Topics for Continued Learning:**
- GPU programming with wgpu for wave modeling
- XBeach-style morphodynamic modeling
- Phase-resolving models (OpenFOAM bindings)
- Digital twin development for coastal structures
- Real-time computer vision for overtopping detection
- Nature-based solution performance monitoring

This structured plan provides a practical pathway from mathematical foundations to production-ready coastal engineering applications, leveraging Rust's performance and safety features while building the exact skills needed for modern coastal intelligence systems.