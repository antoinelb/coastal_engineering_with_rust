# Week 1

## Day 1 - 2025-06-27

Last year, I had gone through a bit of the rust book and know some of what the specifics of rust are from various sources over the years, so I decided to go right to linear wave theory and use the final data integration to remind myself of basic rust programming.

### Notes on Sorensen chapter 2-3
#### 2.1 Introduction
- typical wave periods
    - wind generated waves have a range of periods from 1s to 30s (5s to 15s for ocean waves)
    - vessel generated waves have a range of periods from 1s to 3s
    - seismically generated waves (tsunamis) have a range of periods from 5 min to 1 hour
    - dominant periods of the tide are around 12 and 24 hours
- height is defined as the distance from the crest to the trough
    - this height is typically less than 3m, but can exceed 6m during storms
    - vessel waves rarely exceed 1m in height
    - at sea, tsunami and tide waves are relatively low (less than 0.6m), but can increase to respectively 3m and 6m
- wind generated waves are complex since they consist of a superimposed multitude of components having different heights and periods
- the simplest and often most useful theory (based on the effort needed) is the two-dimensional small-amplitude or linear wave theory (first presented by Airy in 1845)
#### 2.2 Small-amplitude wave theory
- linearize the equations that define the free surface boundary conditions
- a periodic velocity potential is sought that satisfies the requirements for irrotational flow
- this velocity potential is valid throughout the water column except at the thin boundary layers at the air-water interface and at the bottom
- assumptions
    - the water is homogeneous and incompressible, surface tension forces are negligible; this makes it, so there is no internal pressure or gravity waves affecting the flow, and the surface waves are longer than the length where surface tension effects are important (wave lengths are greater than about 3cm)
    - flow is irrotational (there is no wind effect and the fluid slips freely at the bottom and other fixed surfaces)
    - the bottom is stationary, impermeable and horizontal so the bottom isn't adding or removing energy or reflecting wave energy (nearshore regions can be accommodated if the slope isn't too steep)
    - the pressure along the air-sea surface is constant
    - the wave height is small compared to the wave length and water depth
- parameters with still water level being $z=0$
    - $L$ : wave length
    - $H$ : wave height
    - $T$ : period
    - $d$ : water depth
    - $\eta$ : distance from wave surface to still water level (wave surface profile)
    - $C = \frac{L}{T}$ : celerity
    - $k = \frac{2\pi}{L}$ : wave number
    - $\sigma = \frac{2\pi}{T}$ : wave angular frequency
    - $\frac{H}{L}$ : wave steepness
    - $\frac{d}{L}$ : relative depth
- rearranged equations
    - velocity potential : $\phi = \frac{gH}{2\sigma}\frac{\cosh k(d + z)}{\cosh kd} \sin(kx - \sigma t)$
    - wave surface profile : $\eta = \frac{H}{2} \cos(kx - \sigma t)$ or $\eta = \frac{H}{2} \cos2\pi(\frac{x}{L} - \frac{t}{T})$
    - wave angular frequency : $\sigma^2 = gk \tanh kd$
    - celerity : $C = \frac{\sigma}{k} = \sqrt{\frac{g}{k}\tanh kd} = \sqrt{\frac{gL}{2\pi}\tanh\frac{2\pi d}{L}} = \frac{gT}{2\pi}\tanh\frac{2\pi d}{L}$
    - wave length : $L = \frac{gT^2}{2\pi}\tanh\frac{2\pi d}{L}$ (calculated iteratively)
- these last two equations are known as the dispersion equation
#### 2.3 Wave classification

