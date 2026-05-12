# Relational Invariant Taxonomy

**Robin Macomber, Metatron Dynamics, Inc.**
**May 2026 — First Draft**

---

## Status

**Canonical Reference Document**

This document defines the formal invariants of the ABR/ABRCE
operator kernel. Each invariant is:

- Operator-native (defined in terms of kernel operations only)
- Computable from operator output without projection-layer
  interpretation
- Linked to a theorem, proof, or spectral result
- Accompanied by a declaration of what is preserved and what
  is discarded when the invariant is projected

This taxonomy is referenced by all Metatron Dynamics repositories
and documents. It is not specific to any domain or application.

All definitions are bounded over D. No claim is made beyond D.

---

## Terminology Constraint

The following terms have no formal meaning within this kernel
and do not appear in invariant definitions:

- noise, random, stochastic
- signal, meaningful, significant
- real, artifact, spurious
- convergence (in the object-primary sense of approaching
  a terminal state)
- stability (in the object-primary sense of resistance to
  perturbation)

These are projection-layer interpretations. Invariants are
defined by operator relationships within D, not by semantic
judgments about observed structure.

Where common usage might employ these terms, this document
substitutes operator-native equivalents:

| Projection-layer term | Operator-native equivalent |
|---|---|
| Signal vs noise | Relational variance present under E vs absent under C∘B∘A |
| Convergence | σ² approaching zero under iteration (relational annihilation) |
| Stability | Persistence of a measured invariant across operator applications |
| Meaningful structure | Relational variance attributable to a specific operator |
| Random | Not defined within D — all elements of D are relationally determined (Theorem 1a) |

---

## 1. Relational Content Invariance

### Definition

The relational content of a field x ∈ D is the equivalence
class [x]_τ under translation equivalence. Two fields are
relationally identical if and only if their centered
representations are equal (Proposition 1, Object Error).

Equivalently: the relational content is fully determined by
the edge field E(x) evaluated under any connected declared
topology (Proposition 2, Object Error).

### Formal Statement

For x, y ∈ D:

```
x ~_τ y  ⟺  x̄ = ȳ  ⟺  E(x) = E(y)
    over any connected topology
```

### Preservation

Operator A extracts relational content by construction.
A(x) = A(y) if and only if x ~_τ y.

Any transformation T applied to x before A that preserves
pairwise differences preserves relational content:

```
Admissible pre-A transformations:
  T(x) = x + c     (uniform shift — unit of [x]_τ)
  T(x) = x / s     (uniform scaling — declared unit choice)

Inadmissible pre-A transformations:
  Any T where ∃ i,j: T(x)[i] - T(x)[j] ≠ x[i] - x[j]
  unless declared within M with stated preserved/discarded
  invariants.
```

### Linked Results

- Axiom 1 (Informational translation invariance, Object Error)
- Definition 2 (Relational structure, Object Error)
- Propositions 1, 2 (Object Error)
- Pre-Operator Transformation Constraint (README, §11 operators.rs)

---

## 2. Index-Local Null Space

### Definition

The index-local null space is the set of relational distinctions
invisible to any operator T where T(x)[i] = f_i(x[i]) for all i.

### Formal Statement

For any uniformly index-local operator T, and any x, y ∈ D with
identical marginal distributions but different relational
structure:

```
T(x) and T(y) have identical marginal distributions.
T cannot distinguish x from y.
```

### Preservation

The null space is a structural property of the operator class.
It is not reducible by refinement, increased resolution, or
algorithmic sophistication within the index-local class
(Theorem 3, Object Error §7.3).

Any detection functional composed of index-local operations
and symmetric aggregation shares this null space.

### Significance for Invariant Evaluation

Any invariant defined in this taxonomy must be evaluated by
a measure that is NOT index-local. If an invariant can be
computed by an index-local operator, it is not a relational
invariant — it is a marginal property.

### Linked Results

- Theorem 1 (Relational null space, Object Error)
- Theorem 3 (Structural invisibility, Object Error)
- Definition 3 (Index-local operator, Object Error)

---

## 3. R-Sustained Circulation

### Definition

R-sustained circulation is the relational variance present
in the output of E but absent in the output of C∘B∘A (or
B∘A in V4). It is the component of the edge field's variance
attributable uniquely to R's antisymmetric coupling.

### Formal Statement

```
V3:  Γ(x, ρ) = σ²(C(R(B(A(x)), ρ))) - σ²(C(B(A(x))))
V4:  Γ(x, ρ) = σ²(R(B(A(x)), ρ))    - σ²(B(A(x)))
```

R-sustained circulation is present when Γ(x, ρ) > 0.

### Interpretation

Γ > 0 does not mean R "detects signal" or "extracts
information." It means: the relational variance of the
edge field under the full operator composition exceeds
the relational variance under the composition without
antisymmetric circulation.

Γ = 0 means: R's coupling does not produce additional
relational variance beyond what B∘A (or C∘B∘A) already
produces. This occurs when the edge field after B has
no antisymmetric component along R's declared coupling
directions — R's coupling operation has no asymmetry
to operate on.

### Properties

- Comparative: defined as a difference between two
  operator compositions
- Bounded: Γ ≤ σ²(E) (R cannot produce more variance
  than the total)
- Operator-native: no projection-layer terms
- Experimentally computable: both σ² values are computed
  from the same input field
- Non-negative: by Theorems 5 and 6, R sustains or
  increases relational variance for nontrivial fields
  with ρ ∈ (0, 0.5]

### Linked Results

- Theorem 5 (Relational collapse without R, Object Error)
- Theorem 6 (Sustained relational dynamics with R, Object Error)

---

## 4. Edge Sign Structure

### Definition

Edge sign structure is the set of signs {sgn(e[i])} across
all edges in an edge field. It encodes the direction of each
pairwise relation: which of the two adjacent nodes has the
larger value.

### Formal Statement

For an edge field e over topology T:

```
S(e) = { sgn(e[i]) | i ∈ edges(T) }

where sgn(x) = +1 if x > 0, -1 if x < 0, 0 if x = 0
```

### Preservation

- **C preserves:** C is monotone and sign-preserving.
  sgn(C(e)[i]) = sgn(e[i]) for all i. (Object Error §8.2)

- **Magnitude projection destroys:** P_mag(e)[i] = |e[i]|
  discards sign. This is a declared projection with:
  Preserved: gradient magnitude.
  Discarded: edge sign structure (direction of relation).

- **Node-sum projection partially preserves:** P_sum
  aggregates signed edges at each node. Net sign at a
  node reflects the balance of incoming and outgoing
  relational differences. Individual edge signs are
  discarded.

### Significance

Edge sign structure is the component of the edge field
destroyed by any absolute-value or squared operation.
Its presence or absence in operator output is determined
by whether the intervening operations are sign-preserving.
Without it, the edge field represents only gradient
magnitude — "how different are nodes i and j" — not
gradient direction — "which direction is the difference."

### Linked Results

- Object Error §8.2 (C is monotone, sign-preserving)
- Object Error §8.7 (Projection operator declarations)
- Object Error §8.6 (Non-injective transformation principle)

---

## 5. Representation Type Discipline

### Definition

Representation type discipline is the invariant that
NodeField and EdgeField representations are not
interchanged without a declared transition.

### Formal Statement

Within the operator pathway:

```
A is the unique NodeField → EdgeField transition.
B, R operate EdgeField → EdgeField.
C operates EdgeField → EdgeField (V3) or is a declared
  projection (V4).

EdgeField → NodeField occurs only through a declared
Projection with stated preserved and discarded invariants.
```

### Violation Conditions

Representation type discipline is violated when:

- Code operates on NodeField values after A should have
  transitioned them to EdgeField
- An EdgeField is implicitly collapsed to a NodeField
  (one value per cell) without a declared Projection
- Preprocessing in NodeField space alters pairwise
  differences before A
- (V4) Multi-component fields are processed as independent
  scalar pipelines, bypassing the multi-topology edge field

### Significance

This invariant is enforced at compile time in the canonical
Rust implementation (type system prevents NodeField/EdgeField
confusion). In Python implementations, it is a discipline
condition maintained by Origin and verified through the
operator-presence queries declared in Triad §3.9.7.

### Linked Results

- Triad §3.9.5 (What Origin must evaluate per operator)
- Triad §3.9.7 (Origin enforcement during build sessions)
- Object Error §8.3 (Type signatures)
- Object Error §8.6–8.7 (Non-injective transformation
  principle, projection declarations)

---

## 6. Operator Ordering

### Definition

Operator ordering is the invariant that the kernel
composition follows the declared sequence.

### Formal Statement

```
V3:  A → B → R → C → E
     E(x, ρ) = C(R(B(A(x)), ρ))

V4:  A → B → R → E
     E(x, ρ) = R(B(A(x)), ρ(A(x)))
```

### Violation Conditions

Operator ordering is violated when:

- Any operator is applied before its predecessor in
  the declared sequence
- Any operator is omitted without declaration
- Any operator's output is fed to a non-successor
  operator
- R's output is fed back to A without passing through
  a declared Projection and re-entering as a new
  NodeField

### Significance

Operator ordering is not a convention. Each operator's
input requirements are satisfied by the output of the
preceding operator in the declared sequence. A produces
the edge field that B requires as input. B produces the
accumulated edges that R requires for antisymmetric
coupling. Reordering produces structurally different
output even when the component operations are individually
correct.

### Linked Results

- Triad §3.9.6 (C → O → D declaration requirement,
  condition 2)
- Object Error §8.2 (Operator definitions)
- README (Canonical operator order)

---

## 7. B-Admissible Spectral Concentration

### Definition

B-admissible spectral concentration is the invariant
that configurations in the image of B∘A have their
energy concentrated at low-frequency modes relative to
the declared topology.

### Formal Statement

For a ring topology with n nodes, B has spectral
eigenvalues:

```
|λ_m(B)| = 2|cos(πm/n)|
```

B is a low-pass filter: large eigenvalue for small m
(low frequency), vanishing at m = n/2 (Nyquist).

Configurations in Im(B∘A) have mode-m content weighted
by |λ_m(B)|. Energy is concentrated at low frequencies.

### Consequence for Scale Transition

The scale-transition operator Δₖ has spectral eigenvalue:

```
|Δ̂ₖ(m)| = 2ρ · |sin(πm/N)| · |cos(3πm/2N)|
```

For modes where B concentrates energy (small m):

```
|Δ̂ₖ(m)| ≈ 2πρm/N → 0  as m/N → 0
```

B-admissible configurations have vanishing scale-transition
error in the dominant spectral components.

### On Irregular Graphs

On irregular graphs, the Fourier basis is replaced by the
graph Laplacian eigenbasis. B sums same-direction edges
according to the declared adjacency; the spectral
consequence on regular topologies is concentration at
low-frequency modes. The precise spectral characterization
on irregular graphs is a declared open condition.

### Linked Results

- delta_computation.md §5 (Effect of B-admissibility)
- delta_computation.md §6 (ρ·N threshold)

---

## 8. Scale Resonance (ρ·N Regime)

### Definition

Scale resonance is the condition under which the
relational structure produced by the operators at
scale k maps admissibly to scale k-1 through the
scale-transition operator Δₖ, with B-admissibility
suppressing the dominant modes of Δₖ.

### Formal Statement

The ρ·N product determines three regimes:

```
ρ·N << 1:  Scale resonance automatic.
           B dominates spectral distribution.
           Dominant mode error O(ρ/N) → 0 as N → ∞.
           Admissible configurations at scale k map
           coherently to scale k-1.

ρ·N ~ 1:   Transition threshold.
           R redistributes energy toward mid-frequencies
           where |Δ̂ₖ| is largest.
           Scale resonance is disrupted.

ρ·N >> 1:  Nonlinear regime.
           C's saturation dominates.
           Spectral analysis insufficient.
           Mode mixing occurs.
```

### The N Parameter on Irregular Graphs

On a ring, N is simultaneously the node count and the
topological diameter. On irregular graphs, these diverge.
Which governs the ρ·N product is a declared open condition.

Candidates:

- Node count: |V(T)|
- Topological diameter: max shortest path length
- Spectral diameter: 1/λ₁ where λ₁ is the smallest
  nonzero eigenvalue of the graph Laplacian
- Effective propagation radius: the number of B
  applications required for accumulated edge information
  to traverse the graph

The appropriate N may be domain-dependent. Origin declares.

### Linked Results

- delta_computation.md §6 (ρ·N threshold)
- delta_computation.md §9 (Summary table)

---

## 9. Domain Closure

### Definition

Domain closure is the invariant that the output of the
operator composition remains in D.

### Formal Statement

```
V3:  E(x, ρ) ∈ E_D  for all x ∈ D
     where E_D = { e : |e[i]| < 1 for all i }
     Guaranteed by C: |C(e)| < 1 for all e.

V4:  E(x, ρ) is not bounded by the kernel.
     C is a declared projection applied at the application
     layer. Domain closure is a property of the chosen
     projection, not of the kernel.
```

### V3 vs V4

In V3, domain closure is a kernel property — C guarantees
it. In V4, domain closure is a projection-layer property —
the kernel output is unbounded, and the choice of projection
determines whether and how the output is bounded.

This is not a loss of rigor. It is a separation of concerns:
the kernel produces relational structure; the projection
bounds it for application-layer use. The bounding is declared
with preserved and discarded invariants.

### Linked Results

- Object Error §8.2 (C — bounded coherence)
- Object Error §8.9 (Sufficiency, not uniqueness — condition c)
- README (V4: C as declared projection)

---

## 10. Relational Determinacy

### Definition

Relational determinacy is the invariant that every element
of D participates in a fully determined set of pairwise
differences with every other element of D.

### Formal Statement

For all x ∈ D with n ≥ 2, and for all distinct indices i, j:

```
x[i] - x[j] is defined and determined by the field.
```

No element of D is relationally disconnected from any other
element. The relational position of each index is specified
by its differences from all other indices.

### Significance

This is not an assumption. It is a consequence of the
definition of D. Any x ∈ D has n finite real components.
The difference between any two exists and has a definite
value.

Relational determinacy is what makes the concept of
"statistical independence" structurally impossible within D
(Object Error §5.2). Two variables may be uncorrelated under
a given probability model while being relationally determined
within D. The model's factorization is a property of the
representation, not of D.

### Linked Results

- Theorem 1a (Relational determinacy over D, Object Error)
- Object Error §5.2 (Statistical independence as object error)
- Object Error §5.3 (Correlation, independence, and
  relational structure)

---

## 11. Projection Declaration

### Definition

Projection declaration is the invariant that every
non-injective transformation applied to operator output
is explicitly declared with stated preserved and
discarded invariants.

### Formal Statement

Let T be any transformation applied to a relational field
such that ∃ x ≠ y with T(x) = T(y). Then T induces
equivalence classes in the output that are not
distinguishable by any downstream operator.

Every such T must declare:

```
(a) Projection type — which mapping is applied
(b) Preserved invariants — what survives the projection
(c) Discarded invariants — what is destroyed
```

### Violation Conditions

Projection declaration is violated when:

- An edge field is reduced to a node field without
  stating what is preserved and discarded
- A multi-component edge field is collapsed to a scalar
  without declaration
- σ² is computed without stating that it is a scalar
  summary of a structured edge field (σ² is itself
  a declared projection)
- Mean, sum, or any symmetric aggregation is applied
  to operator output without declaration

### Significance

The Object Error is itself a projection declaration
violation: the object-primary encoding P : D → R^{m×n}
is a non-injective transformation applied without
declaring its preserved and discarded invariants.
The entire formal argument (Sections 3–7) follows from
this single undeclared projection.

### Linked Results

- Object Error §8.6 (Non-injective transformation principle)
- Object Error §8.7 (Projection operator declarations)
- Triad §3.9.6 (Origin obligations, condition 5)

---

## Summary Table

| # | Invariant | Formal Criterion | Linked Results |
|---|---|---|---|
| 1 | Relational content | [x]_τ preserved under admissible pre-A transforms | Axiom 1, Props 1–2 |
| 2 | Index-local null space | Structural property of operator class; not reducible | Thms 1, 3 |
| 3 | R-sustained circulation | Γ(x,ρ) = σ²(E) - σ²(composition without R) > 0 | Thms 5, 6 |
| 4 | Edge sign structure | sgn(e[i]) preserved by C; destroyed by magnitude projection | §8.2 |
| 5 | Representation type | NodeField/EdgeField not interchanged without declaration | §8.3, Triad §3.9.5 |
| 6 | Operator ordering | Declared sequence A→B→R→(C→)E not reordered | Triad §3.9.6 |
| 7 | B-admissible spectral concentration | Energy concentrated at low modes; Δₖ suppressed | delta_computation §5 |
| 8 | Scale resonance | ρ·N regime determines scale-transition coherence | delta_computation §6 |
| 9 | Domain closure | V3: kernel property via C. V4: projection-layer property | §8.2, §8.9 |
| 10 | Relational determinacy | All pairwise differences defined and determined in D | Thm 1a |
| 11 | Projection declaration | All non-injective transforms declared with preserved/discarded | §8.6, §8.7 |

---

## Declared Open Conditions

1. **B-admissible spectral concentration on irregular graphs.**
   The Fourier-basis characterization applies to ring/torus
   topologies. Graph Laplacian eigenbasis characterization
   is required for irregular graphs.

2. **Scale resonance N parameter on irregular graphs.**
   Whether node count, diameter, spectral diameter, or
   effective propagation radius governs the ρ·N product.

3. **R-sustained circulation under iterated E.**
   Γ is defined for a single application of E. Under
   iterated application, whether Γ remains positive,
   grows, or stabilizes is domain-dependent and
   ρ-dependent. The spectral interaction between R's
   eigenvalues and C's nonlinear saturation under
   iteration is characterized for the ring
   (delta_computation.md) but open for general topologies.

4. **Invariant completeness.**
   This taxonomy may not be exhaustive. Additional
   invariants may emerge from the topology admissibility
   work and from application to new domains. The taxonomy
   is extensible by declaration.

---

*All definitions bounded over D. No claim beyond D.
This document describes structural properties of the
operator kernel. It does not prescribe interpretation
or application.*
