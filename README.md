# Invariant Relational Kernel

**Metatron Dynamics, Inc.**
**Layer:** Invariant relational constraint discipline
**Domain:** D := { x ∈ ℝⁿ | n < ∞ and |x[i]| < ∞ ∀ i ∈ {0, ..., n−1} }
**Measurement mapping:** M : O → D (declared by Origin before processing)

---

## This Repository

This repository contains the canonical operator definitions for
the invariant relational kernel across two formulations:

- **MD V3 (ABRCE)** — scalar fields on single topologies
- **MD V4 (ABR)** — multi-component fields on multiple topologies

Both formulations are correct and complete within their declared
domains. V4 is a generalization of V3, not a replacement. V3 is
the single-topology special case of V4 where C's projection is
lossless because there is one edge type per cell.

All prior results under V3 — including the Object Error theorems,
the alignment drift demonstrations, and the supply chain early
warning work — remain valid.

---

## Scope

The structure described here does not require adoption. It
describes relational admissibility conditions within **D**
whether or not it is referenced.

All quantifiers in this document are bounded over **D**.
No claim is made beyond **D**.

---

## Shared Foundations

The following properties, constraints, and definitions are
invariant across both V3 and V4.

### Kernel Properties

This kernel is:

- Non-procedural
- Non-semantic
- Non-optimizing
- Non-goal-directed
- Non-ontological
- Domain-bounded
- Non-exhaustive with respect to transformations not declared
  within this kernel

Control, time, reward, optimization, and interpretation are not
declared at the operator layer within **D**. They are projections
of operator output.

### Operator Non-Agency

The operators defined in this kernel, for all x ∈ D:

- do not act
- do not cause
- do not optimize
- do not enforce

They are not mechanisms or agents. They define invariant
conditions of relational admissibility within **D**.

### Pre-Operator Transformation Constraint

No transformation T : D → D may be applied prior to A if T
alters pairwise differences between elements of D, unless T
is declared as part of the measurement mapping M.

Admissible transformations prior to A: T(x) = x + c (uniform
shift) and T(x) = x / s (uniform scaling, declared as unit
choice in M).

All other transformations are inadmissible prior to A unless
declared within M with stated preserved and discarded invariants.

### Admissibility

A statement S is admissible within this kernel iff:

1. All referenced elements are members of D.
2. All quantifiers are bounded over declared subsets of D.
3. All transformations map D into D.
4. All variables are images of observables under M.
5. (V4 only) All non-injective transformations applied to
   operator output are declared with preserved and discarded
   invariants.

### Operator Definitions (A, B, R)

A, B, and R are shared across V3 and V4. Their definitions
are identical in both formulations.

**A — Relational Gradient Extraction**

Extracts directed pairwise differences according to declared
topologies. The unique transition from NodeField to EdgeField.

**B — Local Relational Accumulation**

Same-direction accumulation according to declared topology. No
cross-axis coupling. Directional identity preserved.

**R — Antisymmetric Circulation**

Additive cross-coupling between perpendicular axes (2D) or
between spatial and component topologies (V4 multi-topology).
Antisymmetric. Local. No redistribution or pooling.

### ρ — Per-Cell Circulation Strength

```
ρ[i] = ρ_base × max_grad[i] / (1 + max_grad[i])
```

Per-cell. Derived from A(x). No aggregation.

### Triad Structural Discipline

The Triad separates:

- Origin — constraint articulation
- Generator — realization
- Verifier — invariant detection

Separation reduces compensation within **D**.

### Noise

Noise is not a primitive within this kernel. It is a failure
to specify the source of indeterminacy.

If structure appears as noise in a projection of the kernel's
output, that is a property of the projection, not of the data.

---

## MD V3 — ABRCE

**Status:** Complete. Canonical for scalar fields on single topologies.
**Kernel:** A → B → R → C → E
**Composition:** E(x, ρ) = C(R(B(A(x)), ρ))

### Scope

V3 governs scalar fields on single declared topologies:
1D ring, 2D torus, 3D torus, or any declared connected finite
graph with one scalar value per cell.

### C — Bounded Coherence (Kernel Operator in V3)

In single-topology scalar fields, boundedness and coherence
coincide. One denominator per cell is unambiguous because
there is one relational domain.

```
C(e_d)[i] = e_d[i] / (1 + m[i])
where m[i] = max(|edges at cell i|)
```

Properties:
- Bounded: |output| < 1
- Sign-preserving
- Monotone in each component
- Ratio-preserving across edges at a cell
- No cross-cell coupling

C is a kernel operator in V3. It belongs in the composition.

### Operator Type Signatures (V3)

```
A : D    → ℝⁿ
B : ℝⁿ  → ℝⁿ
R : ℝⁿ × ℝ → ℝⁿ
C : ℝⁿ  → D
E : D × ℝ → D
```

### Canonical Operator Order (V3)

**A → B → R → C → E**

This ordering may not be altered within **D**.

### Validated Results Under V3

- Object Error theorems (Thms 1–6)
- ABRCE Alignment Drift Detection (Phi-3 Mini, 88-step lead,
  31/88 field-fires-before-output-degradation)
- Supply chain early warning (88-step lead)
- Robotics instability detection (82-step lead)
- Relational rate limiter (9 branches → 0)
- Bounded update controller
- Bounded plasticity simulation
- Domain closure demonstrations (DCG)

---

## MD V4 — ABR (Multi-Topology Generalization)

**Status:** Complete. Canonical for multi-component fields on
multiple topologies.
**Kernel:** A → B → R → E
**Composition:** E(x, ρ) = R(B(A(x)), ρ(A(x)))

### Scope

V4 governs multi-component fields on multiple declared
topologies. Two kinds of topology are declared:

**Domain topology:** declared adjacency between cells (ring,
torus, or any connected finite graph).

**Component topology:** declared adjacency between measured
quantities at the same cell. Declared by Origin as part of M.

The kernel operators act on edges over both topologies. Both
edge types are first-class. Neither is derived from the other.

### Why C Leaves the Kernel in V4

In multi-topology fields (spatial + component), boundedness
and coherence are independent structural properties.

- C's shared denominator suppresses quiet edge types when
  one type dominates.
- The coupling present in R's output is destroyed by C's
  magnitude compression.
- R's cross-topology circulation is itself the coherence
  expression for multi-component fields.

Therefore C is a declared projection in V4, applied at the
application layer with stated preserved and discarded
invariants. It is not part of the kernel composition.

V3 is the special case where C's projection is lossless
with respect to structural ratios because there is only
one edge type per cell.

### Declared Projections (V4)

Any reduction of the edge field is a declared projection.
No implicit reduction exists.

**C_per_type** — Per-edge-type bounding.
Preserves: internal ratios within each edge type, sign,
independence between types.
Discards: magnitude relationships between edge types.

**C_shared** — Shared-denominator bounding.
Preserves: cross-type magnitude ratios, sign.
Discards: quiet edge types suppressed when one dominates.

**C_spatial_only** — Bound spatial per-type, scale component
edges to O(1) preserving ratios.
Preserves: spatial coherence per component, component edge
magnitude ratios.
Discards: raw spatial magnitudes.

Other projections admissible provided they declare what they
preserve and discard.

### Component Topology Constructors

- **Complete (all-pairs):** all pairs adjacent.
- **Ring:** component i adjacent to (i+1) mod k.
- **Custom:** Origin declares arbitrary adjacency.

### R in V4 — Cross-Topology Circulation

In addition to spatial cross-axis coupling (V3), R in V4
cross-couples edges across topologies:

- Spatial edges receive component-edge asymmetry from
  spatial neighbors.
- Component edges receive spatial-edge asymmetry at each cell.

All coupling is local, antisymmetric, additive.

R's output is the coupled relational structure in which
coherence is observable. Coherence is a property of R's output,
interpreted by Origin. It is not computed by R.

### Declared Open Conditions (V4)

1. **ρ splitting:** Whether ρ_spatial and ρ_component should be
   independent. Current: single ρ.

2. **Component topology scaling:** For k > 3 components,
   all-pairs may exceed the domain's relational structure.
   Origin declares admissible component adjacency.

3. **B component-edge accumulation:** Whether component edges
   should accumulate spatially is domain-dependent. Origin
   declares.

4. **Spatial dimensionality extension:** 1D ring generalizes
   to 2D torus and 3D torus. Structural form identical; edge
   count scales with dimensionality × component count.

5. **Coordinate frame for component coupling:** R's component
   coupling may require source-native coordinates (e.g.,
   SM/MLT for magnetospheric data). Origin declares.

### Empirical Results Under V4

**1D Multi-Topology Ring (Constructed Data)**
- At 100:1 magnitude ratio, per-type projection preserves
  quiet component at 4.3× independent scalar pipelines.
  Shared-denominator suppresses to 0.3%.
- R sustains inter-component coupling (σ² ratio > 1.0) that
  independent scalar pipelines cannot represent.

**2D Torus — SuperMAG St. Patrick's Day Storm (2015-03-17)**
- All-pairs component topology (N, E, Z) on 36×72 grid.
- R sustains 3.5–4.4× more relational variance than B alone.
- Component edge variance tracks storm phase.
- High-order SH band shows stronger R effects than low-order.
- Vector R produced σ²=1.119 at storm peak (first positive
  result on real magnetospheric data).

---

## Relationship to the Object Error

The Object Error paper (object_error.md) establishes:

- Informational content of measured data is relational (Axiom 1)
- Index-local operators cannot detect relational structure (Thm 1)
- Object-primary encoding destroys relational structure (Thm 1b)
- Index-local modeling diverges monotonically (Thm 2)
- Standard validation cannot detect the divergence (Thm 3)
- Non-index-local operators on edge fields detect it (Thm 4)
- Relational dynamics persist under R; without R, σ² → 0 (Thms 5, 6)

These results are proved for the 1D scalar case (V3 ABRCE).
The V4 ABR generalization preserves all theorems. Theorems 1–4
do not reference C. Theorems 5 and 6 concern R's role; the σ²
diagnostic is applied through a declared projection, which is
stated as part of the experimental setup.

---

## Repository Structure

```
README.md                   — This file
operators.rs                — Canonical Rust implementation
                              (V3 scalar + V4 multi-topology,
                               projections, diagnostics, tests)
multi_topology_1d_test.py   — V4 1D ring test suite
abr_kernel_test.py          — V4 SuperMAG 2D torus application
src/
  abr/
    operators_2d.py         — V3 2D scalar operators (Python)
    projections.py          — Edge-to-node projections
    types.py                — Type definitions
  data/
    supermag_pipeline.py    — M: station data loader
    grid_interpolation.py   — M: grid embedding
    sh_decomposition.py     — M: SH decomposition + unit scaling
docs/
  object_error.md           — The Object Error (formal paper)
tests/
  test_operators.py         — Scalar kernel verification (V3)
```

---

## References

- Macomber, R. (2026). Invariant Relational Evolution over
  Bounded Domains. arXiv:2601.22389.
- object_error.md — The Object Error: A Formal Argument.

---

*The structure described above does not require adoption.
It describes relational admissibility conditions within D.
Interpretation and engagement remain projection-layer decisions.
All statements are bounded over D. No claim is made beyond D.*
