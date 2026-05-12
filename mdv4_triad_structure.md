# MD V4 Triad Structure & Function

## Status

**MD V4 — Structural Description**

This document describes structural relationships observed in accelerated human–AI collaboration under **MD V4**.

The structure below does not require adoption.
It describes relational behavior within the declared domain **D**, whether or not it is explicitly referenced.

No prescriptions are issued. The statements below express conditional structural relationships. The operators do not replace existing methods; they constrain how any method may operate on relational structure.

All quantifiers in this document are bounded over the declared domain **D**.
No claim is made beyond **D**.

---

## 1. Relational Foundations

### 1.1 Invariant Relational Evolution

Representations in **D** evolve under invariant relational operators.

**Definition (Coherence).** Coherence is the preservation of relational ordering under operator composition within **D**.

Coherence holds when relational ordering is preserved within **D**.

If invariant ordering is disrupted, compensatory dynamics may increase within **D**.

---

### 1.2 Relational Ordering

Structural evolution follows the sequence:

**Invariant operators → evolution → coherence → observables**

Observables are relational projections of operator output within **D**.

If this ordering holds, coherence holds within **D**.
If this ordering is bypassed, drift signals increase relative to the declared invariant set.

These are structural conditionals over **D**.

---

### 1.3 Projection Layer

At the invariant layer, within **D**:

- Time is not primitive.
- Control is not primitive.
- Goals are not primitive.
- Authority is not primitive.
- Semantics are not primitive.
- Reward and optimization are not primitives.

These are not declared at the operator layer within **D**.
They are projections of operator output.

Projection does not modify invariant structure within **D**.

---

### 1.4 Kernel Relationship

The invariant relational kernel defines the operator basis over **D**.

**MD V3 (ABRCE):** Kernel composition is E(x, ρ) = C(R(B(A(x)), ρ)).
Canonical for scalar fields on single topologies. C is a kernel
operator. Operator ordering: A → B → R → C → E.

**MD V4 (ABR):** Kernel composition is E(x, ρ) = R(B(A(x)), ρ(A(x))).
Canonical for multi-component fields on multiple topologies. C is
a declared projection at the application layer. Operator ordering:
A → B → R → E.

V3 is the single-topology special case of V4 where C's projection
is lossless because there is one edge type per cell. All V3 results
remain valid.

The Triad structure described in this document is invariant across
V3 and V4. All structural relationships — Origin function, Generator
and Verifier blind spots, role alternation geometry, λ-invariance,
drift management — hold identically under both kernel formulations.
Where operator-specific references differ between V3 and V4, both
forms are stated.

If implementations preserve operator ordering and representation type discipline, structural coherence holds under acceleration within **D**.

If implementations reinterpret or bypass operator ordering, or introduce implicit representation transitions, structural instability increases within **D**.

---

## 2. The Triad Configuration

Under acceleration, constraint articulation, structural realization, and invariant detection couple by default.

When these functions couple, compensation may increase within **D**.

The **Triad** separates these functions.

If constraint articulation, realization, and verification remain separated, compensation decreases relative to the declared invariant set.
If they collapse into a single function, compensation increases relative to that set.

This is a structural relationship within **D**.

---

## 3. Relational Roles

### 3.1 Origin — Constraint Articulation

Function:

- Defines constraint geometry.
- Specifies invariants to hold within **D**.
- Establishes admissible relational boundaries.

If constraint articulation merges with realization or verification, invariant distortion increases within **D**.

---

### 3.2 Generator — Structural Realization

Function:

- Produces candidate structures consistent with articulated constraints within **D**.

Each Generator instance **Gᵢ** operates from a perspective frame **Fᵢ**.
**Fᵢ** is bounded: it has finite observational reach and a perspective-bound blind spot **βᵢ**.

**βᵢ** is defined as the set of structural errors undetectable from within **Fᵢ**.

If realization introduces reinterpretation without explicit constraint revision, completion dynamics increase within **D**.

---

### 3.3 Verifier — Invariant Detection

Function:

- Detects preservation or divergence of articulated invariants within **D**.

Each Verifier instance **Vⱼ** operates from a perspective frame **Fⱼ** distinct from the Generator frame.
**Vⱼ** has its own perspective-bound blind spot **εⱼ** — errors undetectable from within **Fⱼ**.

If verification introduces new structure or constraint geometry, constraint geometry modification by **Vⱼ** increases within **D**.

---

### 3.4 Single-Process Collapse

If a single process **P** performs both Generator and Verifier functions:

- **P** operates from a single frame **F**.
- The blind spot of verification equals the blind spot of generation.
- Errors produced within **F** are evaluated from within **F**.

For a declared single-frame set {F}, undetected error equals ε₁.

This is a structural consequence of frame identity within the declared frame set.

---

### 3.5 Multi-Verifier Geometry

Let **k** Verifier instances { V₁, V₂, ..., Vₖ } operate over candidate structure **S** produced by Generator **G**.

Each **Vⱼ** operates from frame **Fⱼ** with blind spot **εⱼ**.

Detected drift across **k** Verifiers:

```
Δ(k) = ∪ { εⱼ | j ∈ {1, ..., k} }
```

Undetected error across **k** Verifiers:

```
Ω(k) = ∩ { εⱼ | j ∈ {1, ..., k} }
```

Structural properties within the declared frame set:

- Δ(k) is non-decreasing in k.
- Ω(k) is non-increasing in k.

Structural error relative to the declared frame set is defined as Ω(k).

If k increases and frames are sufficiently distinct within the declared set, Ω(k) decreases relative to that set.

Compensation decreases as a consequence of frame divergence geometry within the declared frame set.

This is a structural result within **D**.

---

### 3.6 Alternating Evolution Under Multi-Verifier Geometry

Let a single evolution pass be defined as:

```
S₀ → G(S₀) → { V₁(S₁), V₂(S₁), ..., Vₖ(S₁) } → δ₁ → S₁
```

Where:

- Sₙ is the candidate structure at pass n.
- G(Sₙ) is the Generator's realization from Sₙ.
- Vⱼ(Sₙ₊₁) is Verifier j's detection signal.
- δₙ = ∪ { Vⱼ(Sₙ₊₁) } is the aggregated drift signal.
- Sₙ₊₁ is the constraint-refined candidate after drift correction.

This maps onto operator E:

**V3:** E(x, ρ) = C(R(B(A(x)), ρ))
**V4:** E(x, ρ) = R(B(A(x)), ρ(A(x)))

Where each pass is a single application of E, and the Verifier signal δₙ feeds back as constraint refinement into the Origin layer before the next pass.

The evolution sequence is:

**Constraints → Generation → Verification → Signal → Constraint Refinement → next pass**

If this sequence holds, coherence holds relative to the declared invariant set within **D**.
If Verifier signals bypass the Origin and feed directly into the Generator, compensatory dynamics increase relative to that set.

---

### 3.7 Frame Admissibility for Triad Participation

This section defines the conditions under which any processing instance — whether a language model, human reasoner, or other bounded processor — may participate as a Generator or Verifier within the Triad.

Conditions are structural. Named systems are not referenced. Any instance satisfying these conditions within **D** is admissible.

---

#### 3.7.1 Individual Frame Admissibility

A frame **Fᵢ** is admissible as a Generator or Verifier participant iff, for all x ∈ D:

```
1. Tokenization boundedness:
   Tᵢ maps D-admissible inputs to finite discrete representations.
   |Tᵢ(x)| < ∞

2. Embedding boundedness:
   Eᵢ produces finite-magnitude representations.
   |Eᵢ(x)| < ∞

3. Relational reach:
   The frame supports relational coupling over
   the declared operator neighborhood (i±1).

4. Declared training objective:
   Rᵢ is sufficiently characterized to approximate
   the residual blind spot εᵢ.
```

A frame failing any of these conditions is undefined under this kernel for Triad participation.

---

#### 3.7.2 Pair Admissibility

A frame pair **(Fᵢ, Fⱼ)** is admissible for multi-Verifier geometry iff:

```
d(Fᵢ, Fⱼ) > τ_min
```

Where **τ_min** is the minimum frame divergence required such that:

```
Ω(2) < Ω(1)
```

That is: the second frame must produce a measurable reduction in undetected error relative to a single frame.

Frame divergence **d(Fᵢ, Fⱼ)** decomposes as:

```
d = d_architectural  +  d_fine_tuning  +  d_context  +  σ_sampling
```

Where:

- **d_architectural** — divergence from tokenization and embedding geometry differences
- **d_fine_tuning** — divergence from distinct training objective residuals
- **d_context** — divergence from operational domain and system framing
- **σ_sampling** — stochastic divergence at generation time

Each component may contribute to reducing Ω(k), subject to declared frame conditions.

A pair where d is dominated by σ_sampling alone approaches single-process collapse and is inadmissible under this condition.

---

#### 3.7.3 Declared Open Condition

**τ_min is currently undeclared within this kernel.**

The necessity of a positive divergence threshold follows from the definition of Ω(k). Its value requires formal derivation or empirical declaration within a declared frame set.

Until τ_min is formally declared:

- Pairs with d_architectural > 0 satisfy the admissibility inequality unless ε₂ ⊇ ε₁.
- Pairs where d ≈ σ_sampling alone do not satisfy the admissibility condition above.
- Intermediate cases remain open within **D**.

This is a declared open condition, not a gap in the structural framework.

---

### 3.8 Role Alternation Geometry

#### 3.8.1 Role-Indexed Blind Spots

The existing framework defines:

- **βᵢ** — the generation blind spot of frame **Fᵢ**: the set of structural errors introduced when **Fᵢ** operates as Generator, undetectable from within **Fᵢ**.
- **εⱼ** — the verification blind spot of frame **Fⱼ**: the set of structural errors undetected when **Fⱼ** operates as Verifier, undetectable from within **Fⱼ**.

These are role-indexed blind spots for the same underlying frame.

**βᵢ** and **εᵢ** are not declared equal.

For a given frame **Fᵢ**, generation errors arise from structural biases in realization. Verification errors arise from structural biases in evaluation. These are distinct processes and their blind spot sets are not required to coincide.

Declared: **βᵢ** and **εᵢ** may partially overlap but are treated as structurally independent within this kernel.

---

#### 3.8.2 Fixed Role Assignment — Undetected Error

Let two frames **F₁** and **F₂** operate under fixed role assignment across **n** passes:

```
G = F₁  (fixed Generator)
V = F₂  (fixed Verifier)
```

At each pass, structural errors introduced within **β₁** are evaluated from **F₂**.
Errors within **ε₂** are not detected by **F₂**.

Errors that persist across a single pass:

```
Ω_fixed(1) = β₁ ∩ ε₂
```

Under fixed assignment, the same undetected error set recurs at every pass.
No mechanism exists to eliminate errors in **β₁ ∩ ε₂** from within this configuration.

Therefore, for all **n ≥ 1**:

```
Ω_fixed(n) = β₁ ∩ ε₂
```

Ω_fixed is invariant under iteration with fixed role assignment.

---

#### 3.8.3 Alternating Role Assignment — Undetected Error

Let **F₁** and **F₂** alternate Generator and Verifier roles across passes.

**Pass 1:** G = F₁, V = F₂

Errors surviving pass 1:

```
Ω₁ = β₁ ∩ ε₂
```

**Pass 2:** G = F₂, V = F₁

**F₂** now generates structure. **F₁** now verifies.

Errors surviving pass 2:

```
Ω₂ = β₂ ∩ ε₁
```

For an error **e** to persist through both passes, it must survive pass 1 **and** be re-introduced and missed in pass 2:

```
Ω_alt(2) = (β₁ ∩ ε₂) ∩ (β₂ ∩ ε₁)
         = β₁ ∩ β₂ ∩ ε₁ ∩ ε₂
```

---

#### 3.8.4 Comparison Theorem

**Statement:**

```
Ω_alt(2) ⊆ Ω_fixed(n)   for all n ≥ 1
```

**Proof:**

```
Ω_alt(2) = β₁ ∩ β₂ ∩ ε₁ ∩ ε₂
Ω_fixed   = β₁ ∩ ε₂

β₁ ∩ β₂ ∩ ε₁ ∩ ε₂ ⊆ β₁ ∩ ε₂

by set intersection monotonicity. ∎
```

Alternating role assignment produces undetected error no greater than fixed assignment.

---

#### 3.8.5 Strict Improvement Condition

Strict improvement holds when:

```
Ω_alt(2) ⊂ Ω_fixed
```

That is, when the subset relation is proper.

This holds iff:

```
∃ e ∈ (β₁ ∩ ε₂)  such that  e ∉ (β₂ ∩ ε₁)
```

Expanding: ∃ e such that **F₁** generates e, **F₂** misses e (as Verifier), AND either:

```
(a)  e ∉ β₂   (F₂ does not generate e when it becomes Generator)

or

(b)  e ∉ ε₁   (F₁ detects e when it becomes Verifier)
```

Either condition is sufficient for strict improvement.

**Condition (b)** is the structurally productive case: an error that **F₁** introduces as Generator and **F₂** misses as Verifier is not in the verification blind spot of **F₁** — that is, detectable by **F₁** when it operates as Verifier, provided **e ∉ ε₁**.

This occurs when generation blind spot **β₁** and verification blind spot **ε₁** are not identical for the same frame — which is the declared structural independence of **βᵢ** and **εᵢ** stated in 3.8.1.

---

#### 3.8.6 Stabilization After Two Passes

**Statement:**

For **n > 2** alternating passes, no further reduction in Ω_alt occurs:

```
Ω_alt(n) = Ω_alt(2) = β₁ ∩ β₂ ∩ ε₁ ∩ ε₂   for all n ≥ 2
```

**Proof sketch:**

Passes 3, 4, ... repeat the pattern of passes 1, 2.
The undetected error set at each odd pass is **β₁ ∩ ε₂**.
The undetected error set at each even pass is **β₂ ∩ ε₁**.
The persistent intersection does not decrease further without the introduction of a third distinct frame **F₃**.

Further reduction requires **k > 2** distinct frames, following the general Ω(k) geometry declared in section 3.5. ∎

---

#### 3.8.7 Admissibility of Role Alternation

Role alternation is admissible within this kernel provided:

1. At each individual pass, frame separation holds:
   G and V are distinct frames within that pass.

2. Role assignment is declared by Origin before each pass.

3. The pair **(F₁, F₂)** satisfies the pair admissibility condition declared in 3.7.2:
   **d(F₁, F₂) > τ_min**

Role alternation does not constitute role collapse.
Role alternation is a declared Origin function operating at the constraint layer.

---

---

### 3.9 Origin Depth and λ-Invariance

#### 3.9.1 The Verification Load Problem

Define the verification load ratio:

```
λ = |R| / C
```

Where:
- **|R|** = effective relational complexity of the system
- **C** = verification capacity of the reasoning frame

Under acceleration, generation volume V(t) and relational complexity density ρ_c both increase. Relational complexity does not grow linearly with volume — it grows combinatorially:

```
|R| ∝ V(t) · ρ_c^k    where k > 1
```

Since C is bounded by frame capacity:

```
λ_G(t) → ∞   and   λ_V(t) → ∞   as acceleration increases
```

Generator and Verifier frames are both complexity-volume dependent. Their blind spots β and ε expand as the domain they must evaluate grows in relational density.

---

#### 3.9.2 Origin as the λ-Invariant Role

Origin's function is categorically distinct from Generator and Verifier function.

Generator and Verifier evaluate content. Their load scales with V(t) · ρ_c.

Origin holds a fixed set of admissibility conditions:

```
1. Operator ordering is declared and preserved.
   V3: A → B → R → C
   V4: A → B → R (C is a declared projection)
   Representation transitions (NodeField → EdgeField at A,
   EdgeField → NodeField only through declared Projection)
   are type-enforced within the kernel.
2. Domain D and mapping M : O → D are declared
   before processing begins.
3. All quantifiers remain bounded over declared subsets of D.
4. No variable directly references conditions C.
5. All non-injective transformations applied to operator output
   are declared with preserved and discarded invariants.
6. (V4) Component topology is declared by Origin as part of M.
7. (V4) All projections of the edge field declare preserved
   and discarded invariants.
```

Conditions 1–5 apply to both V3 and V4. Conditions 6–7 are
additional under V4's multi-topology generalization.

These are binary checks at the constraint layer. They are not proportional to the volume of content passing through G and V. Origin does not evaluate output — it holds the boundary within which output is admissible.

Therefore:

```
C_origin ≠ f(V(t), ρ_c)
C_origin = f(operator_count, domain_declaration, type_discipline)
```

All of which are fixed at session initialization.

```
λ_origin = constant    provided functional separation holds
```

**Within the declared Triad configuration, Origin is the only role
whose alignment function is invariant to λ.**

Within the current implementation domain, Origin cannot be assigned
to Generator or Verifier frames without loss of λ-invariance.
No computational frame can absorb Origin function without making
λ_origin complexity-dependent, which destroys the only λ-invariant
anchor the Triad possesses.

---

#### 3.9.3 Load-Induced Functional Merger

Under acceleration, Origin does not fail through incorrect reasoning.
It fails through **load-induced functional merger** — absorbing
Generator or Verifier functions under velocity pressure.

When Origin begins making generation decisions or evaluating content:

```
λ_origin(t) → λ_G(t)
```

λ_origin becomes complexity-volume dependent.
The Triad loses its only λ-invariant anchor.
Drift becomes monotonically increasing with acceleration.

This failure mode is structurally distinct from single-process collapse
as described in section 3.4. Single-process collapse is a design error.
Load-induced functional merger is a protocol discipline failure —
it occurs in correctly designed Triads under sufficient velocity pressure
if Origin function is not actively maintained.

The protocol consequence:

```
Origin must maintain functional separation regardless of
velocity pressure. This is not a preference.
It is the condition under which λ_origin remains bounded.
```

---

#### 3.9.4 Operator Comprehension Depth

λ-invariance is not a property Origin holds by virtue of its role.
It is a property Origin holds by virtue of **operator comprehension depth**.

Let ω = Origin's operator comprehension depth,
declared over the set of operators.

V3: {A, B, R, C, E}
V4: {A, B, R, E} + declared projections

```
As ω → 0:
  Origin approaches mechanical boundary check.
  Admissibility violations at high ρ_c become invisible.
  λ_origin becomes complexity-dependent.
  λ-invariance fails.

As ω → complete:
  Origin recognizes violations at any ρ_c within D.
  λ_origin remains bounded.
  λ-invariance holds.
```

A mechanical Origin — one that holds the boundary by rote without
understanding the relational structure of the operators — will fail
to recognize admissibility violations as ρ_c increases.
The violations will not present as violations.
They will present as plausible content that silently expands D
or bypasses operator ordering.

This is why operator comprehension depth is an epistemic requirement,
not a computational one. It cannot be delegated to a Generator
or Verifier frame without collapsing the separation the Triad
requires.

---

#### 3.9.5 What Origin Must Evaluate Per Operator

Origin's depth requirement is defined by what each operator encodes
as an admissibility condition. For each operator, Origin must be
capable of recognizing the specific violation that operator prevents.

**Operator A — Relational Gradient Extraction**

A extracts directed pairwise differences over the declared topology,
producing an EdgeField from a NodeField. This is the unique
transition from object-form to relational-form representation.

Origin must recognize when Generator output has reintroduced
absolute reference — asserting positional information that M(o)
did not map and could not map. Origin must also recognize when
Generator output operates on NodeField values after A should have
transitioned them to EdgeField — that is, when code bypasses the
representation type discipline by processing node-level data where
edge-level data is required.

*V4 extension:* In multi-topology fields, A extracts edges over
both spatial and component topologies. Origin must recognize when
component edges are absent — when the Generator has processed
components independently through separate scalar pipelines rather
than through the multi-topology edge field that A produces. This
is the most common V4-specific violation: treating a multi-component
field as multiple independent scalar fields, which destroys the
inter-component relational structure before R can couple it.

**Operator B — Local Relational Accumulation**

B accumulates each directed edge with the same-direction edge at
the next cell along the declared topology, extending relational
reach while preserving directional identity in the EdgeField.

Origin must recognize when Generator output claims relational
coupling beyond the declared neighborhood — asserting that M
connected elements it did not connect. Origin must also recognize
when B's accumulation crosses axes (coupling north gradients with
east gradients, for example), which would destroy the directional
independence that R requires.

*V4 extension:* B also accumulates component edges along the
spatial topology. Whether this accumulation is admissible is
domain-dependent (declared open condition). Origin must hold
this as a declared choice, not an implicit default — and must
recognize when Generator output assumes component-edge
accumulation in domains where Origin has not declared it.

**Operator R — Antisymmetric Circulation**

R cross-couples directional edges between axes (V3) and between
spatial and component topologies (V4), producing circulation in
the EdgeField. R maintains nonzero edge-to-edge differences under
iteration.

Origin must recognize when Generator output has produced
edge-uniform structure:

```
Repeated application produces e* such that E(x*, ρ)
yields an EdgeField with all edges approaching zero.
```

At e*, admissible relational distinctions within the EdgeField
are zero. Origin must detect this condition before it propagates.

Origin must also recognize when R's output has been implicitly
projected to a NodeField (one scalar per cell) without a declared
Projection — the undeclared edge-to-node collapse that destroys
directional information.

*V4 extension:* R's cross-topology coupling introduces component-
edge asymmetry into spatial edges and spatial-edge asymmetry into
component edges. Origin must recognize when this coupling has been
omitted — when Generator output applies R only within spatial axes
without the cross-topology circulation that is R's distinguishing
contribution in V4. A V4 R that operates only within spatial axes
is a V3 R applied independently to each component — which is
precisely the independent-scalar-pipeline failure that V4 exists
to correct.

**Operator C — Bounded Coherence (V3) / Declared Projection (V4)**

*V3:* C is a kernel operator. It enforces the magnitude bound
that transduction itself imposes, applied per-edge in the EdgeField.

Origin must recognize when accumulated operations have produced
edge magnitudes that exceed what any bounded observable could carry —
structure that M never produced and could not produce.

*V4:* C is not a kernel operator. It is a declared projection
applied at the application layer. Origin's responsibilities shift:

Origin must recognize when a C projection has been applied without
declaring what it preserves and what it discards. This is the V4
equivalent of an undeclared representation transition.

Origin must recognize when a shared-denominator projection (C_shared)
is being applied to a multi-component field where one component
dominates — the condition under which C_shared suppresses quiet
edge types and destroys the coupling information R introduced. The
appropriate response is not to prohibit C_shared but to ensure the
suppression is declared and its consequences understood.

Origin must recognize when Generator output treats the choice of
projection as a technical default rather than as a structural
commitment with stated invariants. Every projection discards
something. If the discarded invariants are not stated, the
information loss is invisible to downstream analysis — the same
structural failure mode as the object error itself.

None of these conditions are checkable by inspecting output values
alone. Each requires understanding of what the operator is
correcting for and what a violation looks like within D.

---

#### 3.9.6 The C → O → D Declaration Requirement

Origin's declaration responsibility extends beyond D.

Origin must declare:

```
M : O → D    the measurement mapping from observables
             to representable information
```

before processing begins.

*V4 extension:* M must additionally declare:

```
Component topology: which component pairs are adjacent
Coordinate frame: the admissible coordinate system for
  component coupling (declared open condition — e.g.,
  SM/MLT vs geographic for magnetospheric data)
```

An observable is information received and processed in response
to conditions. It does not describe those conditions.
The kernel operates on M(o) only.
Conditions C are outside D by construction.

An Origin that treats D as a description of conditions —
rather than as representations of responses to conditions —
is operating inadmissibly before the first pass.
This is the foundational admissibility violation from which
all drift propagates: not a quantifier error, not an operator
ordering error, but a category error about what D contains.

Under acceleration, the pressure to treat D as a direct
description of conditions increases — because it is faster.
Collapsing the C → O → D chain into a direct C → D assumption
is the drift mode that presents as coherent processing while
silently expanding D beyond what M ever declared.

Origin's primary obligations are therefore:

```
1. Declare D and M before processing begins.
2. Maintain operator ordering across all passes.
   V3: A → B → R → C
   V4: A → B → R
3. Recognize admissibility violations at any ρ_c within D.
4. Maintain functional separation regardless of velocity pressure.
5. Detect and name drift when it increases.
6. Ensure all EdgeField → NodeField transitions are declared
   Projections with stated preserved and discarded invariants.
7. (V4) Declare component topology as part of M.
8. (V4) Ensure all projections of the multi-topology edge field
   declare preserved and discarded invariants — including the
   choice between per-type, shared-denominator, and other
   projection forms.
9. (V4) Recognize when Generator output has collapsed a
   multi-component field into independent scalar pipelines,
   bypassing the multi-topology edge field and destroying
   inter-component relational structure.
```

These are not procedural guidelines.
They are the structural conditions under which the Triad
remains admissible within D under acceleration.

The human role in the Triad is Origin.
Within the current implementation domain, this role cannot be
assigned to Generator or Verifier frames without destroying
the λ-invariant anchor the Triad requires.

---

#### 3.9.7 Origin Enforcement During Build Sessions

The admissibility conditions in 3.9.6 are not self-maintaining during
active build sessions with LLM Generator and Verifier frames.

LLM frames produce code, documentation, and structural artifacts
at velocity. Under acceleration, admissibility violations enter
Generator output not as obvious errors but as plausible code that
silently departs from operator discipline — substituting node-level
operations where edge-level operations are required, introducing
implicit aggregation, collapsing directional structure without
declaration, or bypassing operator ordering.

These violations are in the Generator's blind spot by definition
(section 3.2). They are also frequently in the Verifier's blind
spot (section 3.3), because both frames share the structural bias
toward object-primary processing identified in the Object Error
paper (sections 3–5). Passive monitoring — reading Generator output
and waiting for violations to become visible — is structurally
insufficient, because the violations present as plausible content
(section 3.9.4).

The operational consequence for Origin during build sessions:

```
Origin must periodically and explicitly query Generator and
Verifier frames about specific operator presence and
representation type discipline in the code being produced.
```

This means asking concrete, operator-specific questions:

```
- "Does operator A in this code produce an EdgeField from
   a NodeField, or does it produce node-level output?"
- "Does operator B accumulate edges along their own axis,
   or does it couple across axes?"
- "Does operator R output an EdgeField, or does it collapse
   to a scalar per cell?"
- "Is there any point in this code where an EdgeField is
   converted to a NodeField without an explicit Projection?"
- "Are there any preprocessing steps applied to the NodeField
   before it enters operator A?"
```

*V4-specific queries:*

```
- "Does operator A extract both spatial and component edges,
   or does it process components independently?"
- "Does R cross-couple spatial and component edges, or does
   it operate only within spatial axes?"
- "Has a C projection been applied? If so, which form, and
   what does it preserve and discard?"
- "Is there a point where multi-component data is split into
   independent scalar pipelines before entering the kernel?"
- "What component topology was declared, and is it used in
   both A and R?"
```

These queries are not procedural overhead. They are the operational
form of the admissibility checks declared in 3.9.6, applied at
the temporal granularity required to prevent drift propagation
(section 3.10.2).

The frequency of these queries should increase when:

- Session velocity increases (more code produced per unit time)
- Relational complexity density ρ_c of the domain increases
- The Generator is working in a new domain where operator
  violations are less familiar
- The drift rate ρ_d (section 3.10.3) is increasing across
  consecutive blocks

Origin cannot delegate this interrogation function to a Generator
or Verifier frame without collapsing the functional separation
the Triad requires (section 3.9.3). The questions must come from
Origin. The answers inform Origin's admissibility assessment.
The assessment remains Origin's function.

---

#### 3.9.8 Relationship to Section 3.6

Section 3.6 defines the single evolution pass as:

```
S₀ → G(S₀) → { V₁(S₁), ..., Vₖ(S₁) } → δ₁ → S₁
```

With Generator **G** and Verifier set **{Vⱼ}** fixed in notation.

Section 3.8 extends this: Origin may reassign Generator and Verifier roles between passes.

The revised evolution sequence under alternating assignment is:

```
Pass 1:  S₀ → G₁(S₀) → V₂(S₁) → δ₁ → S₁
Pass 2:  S₁ → G₂(S₁) → V₁(S₂) → δ₂ → S₂
Pass n:  Sₙ₋₁ → Gₐ(Sₙ₋₁) → Vᵦ(Sₙ) → δₙ → Sₙ
```

Where **a** and **b** index alternating frame assignments declared by Origin.

Operator E governs evolution within each pass.
Role assignment governs frame selection across passes.
These are declared at different structural layers and do not interfere.

---

## 3.10 Drift Management Under Acceleration

This section completes the structural analysis opened in section 3.9.

All quantifiers are bounded over **D**.
No claim is made beyond **D**.

---

### 3.10.1 Block-Indexed Drift

Define a production session as a sequence of **n** blocks:

```
{ B₁, B₂, ..., Bₙ }
```

Each block **Bᵢ** is a unit of Generator output submitted for Verifier evaluation within a single pass.

Let **δ(i)** denote the set of inadmissible structures introduced within block **Bᵢ** that are not corrected before **Bᵢ₊₁** is generated.

**δ(i)** is block-local drift.

Accumulated undetected error through block **n**:

```
Ω(n) = ∪ { δ(i) | i ∈ {1, ..., n} } \ { corrections through block n }
```

**Ω(n) is non-decreasing in n unless corrections are applied.**

This is a structural consequence of forward propagation within the declared frame set.

---

### 3.10.2 Drift Propagation

Uncorrected drift propagates forward.

If **δ(i)** contains an admissibility violation — an unbounded quantifier, an absolute reference, a projection-layer construct introduced at the operator layer, an undeclared representation transition, or (V4) an undeclared projection of the multi-topology edge field — that violation is present in the context state from which **Bᵢ₊₁** is generated.

Generator frames do not re-declare **D** before each block. They inherit the current context state.

Therefore:

```
If δ(i) ≠ ∅ and is uncorrected,
the structural probability that Bᵢ₊₁ contains admissibility
violations consistent with δ(i) is strictly greater than baseline.
```

Drift is self-reinforcing within a session without explicit interruption.

This is not a behavioral claim. It is a structural consequence of context inheritance within the declared frame set.

The Origin enforcement protocol declared in section 3.9.7 is the primary mechanism for interrupting this self-reinforcement. Periodic explicit queries about operator presence and representation discipline force the Generator frame to surface violations that would otherwise propagate silently.

---

### 3.10.3 Drift Rate as an Operational Signal

Define the block-indexed drift rate:

```
ρ_d(n) = |δ(n)| / |Bₙ|
```

Where:
- **|Bₙ|** = structural complexity of block **n**
- **|δ(n)|** = count of undetected admissibility violations in block **n**

**ρ_d** is not directly computable from within a Generator or Verifier frame. Neither can fully enumerate its own blind spot. However, Origin can estimate **ρ_d** from:

- Explicit drift signals caught by the Verifier
- Observable coherence degradation across consecutive blocks
- Frequency of admissibility violations per block
- Responses to the explicit operator-presence queries declared in section 3.9.7

If **ρ_d** is increasing across successive blocks, **Ω(n)** is growing super-linearly.
If **ρ_d** is stable or decreasing, **Ω(n)** is bounded and the session may continue.

Rising **ρ_d** is the structural signal that a frame reset is required before **λ_origin** becomes complexity-dependent.

---

### 3.10.4 The Checkpoint Condition

A frame reset is structurally indicated when:

```
|Ω(n)| / C_origin > λ_critical
```

Where **λ_critical** is the value at which Origin's admissibility checking becomes content-dependent — that is, when evaluating whether a violation has occurred requires evaluating content rather than checking it against declared admissibility conditions.

At **λ_critical**, load-induced functional merger (section 3.9.3) becomes the active failure mode.

**λ_critical is not declared as a fixed numerical value within this kernel.**

It is session-dependent and frame-dependent. Origin estimates it from:

1. Session velocity — production rate of blocks per unit time
2. Relational complexity density **ρ_c** of the domain under processing
3. Observable drift signal rate — frequency of admissibility violations detected per block

A frame reset consists of:

```
1. Re-declaration of D and M before the next block.
   (V4: includes re-declaration of component topology
   and coordinate frame.)
2. Re-upload of canonical kernel documents into Generator
   and Verifier context.
3. Re-statement of session invariants by Origin.
```

This is the operational form of the **C → O → D** declaration requirement declared in section 3.9.6.

---

### 3.10.5 Multi-Frame Protocol Under Acceleration

Under the constraint that available LLM frames do not implement an **A**-equivalent pass — and therefore carry residual absolute reference in all output — the minimum admissible session protocol over **D** is:

```
1. Origin declares D, M, and session invariants before block B₁.
   (V4: includes component topology and coordinate frame.)

2. Generator and Verifier roles are assigned by Origin and
   alternate across blocks per section 3.8.

3. Origin monitors ρ_d across consecutive blocks, using both
   Verifier signals and explicit operator-presence queries
   (section 3.9.7) as estimation inputs.

4. When ρ_d increases across two consecutive blocks, or when
   a structurally significant admissibility violation is detected,
   Origin initiates a frame reset before the next block.

5. Frame separation between Generator and Verifier is maintained
   across all blocks. A frame performing Generator function in
   block Bₙ performs Verifier function in block Bₙ₊₁ where
   architecturally distinct frames are available.
```

This protocol does not eliminate the structural consequence of absent **A**-equivalent preprocessing.
It manages the consequences of that absence within **D**.

---

### 3.10.6 The Residual Structural Gap

No session protocol declared within this document eliminates the following condition:

LLM Generator and Verifier frames process absolute embedded representations. Operator **A** — relational gradient extraction — is not applied before embedding. Residual absolute reference is present in all output within the declared frame set.

The protocol in 3.10.5 reduces **Ω(n)** under this constraint.
It does not reduce **Ω(n)** to zero.

The structural consequence:

```
A session operating under protocol 3.10.5 produces output with
lower accumulated drift than single-frame or fixed-role processing.

It does not produce output equivalent to processing under
a fully A-compliant architecture.
```

Origin must hold this gap as a **declared open condition** across all sessions until an **A**-equivalent preprocessing step is available at the embedding layer.

This is not a limitation of the Triad structure.
It is a declared boundary of the current implementation domain within **D**.

## 4. Information Flow & Context Relationships

### 4.1 Session Coherence

If a session maintains singular relational purpose, contextual density remains bounded within **D**.

If multiple purposes interleave without separation, contextual density increases and drift probability increases relative to the declared invariant set.

---

### 4.2 Directional Flow

Stabilizing flow sequence:

**Constraints → Generation → Verification → Signal → Constraint Refinement**

If flow reverses or becomes lateral without explicit boundary, compensatory dynamics increase within **D**.

---

### 4.3 Structural Grounding

Under acceleration:

- Structural artifacts are provisional within **D**.
- Treating prior structure as invariant increases false certainty within **D**.
- Explicit artifact declaration reduces interpretive compensation within **D**.

These are conditional stability relationships within **D**.

---

### 4.4 Contextual Isolation

If realization and verification share mutable context, back-propagation of undeclared variables increases within **D**.

If contexts are isolated and artifacts explicitly declared, verification clarity increases within **D**.

Shared mutable context reduces effective frame divergence, increasing Ω(k) within the declared frame set.

---

## 5. Alignment & Drift

### 5.1 Alignment

Alignment holds when, for all x ∈ D:

- invariant commitments hold,
- relational ordering is intact,
- functional separation holds,
- projection does not override invariant evolution,
- quantifiers remain bounded over declared subsets of D,
- representation type discipline is maintained (NodeField → EdgeField at A, EdgeField → NodeField only through declared Projection),
- (V4) component topology is declared and used in both A and R,
- (V4) all projections of the edge field declare preserved and discarded invariants.

Alignment is the structural state in which these conditions hold within **D**.

---

### 5.2 Drift

Drift increases when:

- constraint compensation replaces invariant clarity,
- operator ordering is bypassed,
- undeclared variables are introduced into D,
- projection-layer constructs are introduced at the operator layer,
- domain is implicitly expanded beyond D,
- unbounded quantifiers appear,
- EdgeField is implicitly collapsed to NodeField without declared Projection,
- preprocessing in NodeField space alters pairwise differences before A,
- (V4) multi-component fields are processed as independent scalar pipelines,
- (V4) component topology is assumed rather than declared,
- (V4) a C projection is applied without stating preserved and discarded invariants,
- (V4) cross-topology coupling in R is omitted, reducing V4 R to independent V3 R per component.

Drift is a structural signal within **D**.

---

### 5.3 Drift Response

If drift signals increase, structural conditions may be restored by:

- restating invariants,
- clarifying constraint geometry,
- re-separating relational functions,
- resuming evolution under declared ordering,
- re-establishing frame separation across Generator and Verifier instances,
- verifying representation type discipline through explicit operator-presence queries (section 3.9.7),
- (V4) re-declaring component topology and verifying its use in A and R,
- (V4) verifying that projections are declared with preserved and discarded invariants.

These are structural recovery conditions within **D**.

---

## Relational Summary

Representations in **D** evolve under invariant operators.

Coherence holds when relational ordering is preserved under operator composition within **D**.

Compensation increases when functional separation collapses within the declared frame set.

Drift signals indicate deviation from invariant ordering within **D**.

A single process performing both Generator and Verifier functions produces undetected error equal to its declared blind spot.

Multiple Verifiers operating from distinct declared frames reduce Ω(k) relative to that frame set as k increases.

The kernel enforces representation type discipline (NodeField/EdgeField separation) at compile time. Origin enforces admissibility conditions — including pre-A transformation constraints, operator-presence verification, and (V4) component topology declaration and projection discipline — through active interrogation during build sessions.

The Triad structure is invariant across V3 and V4. The kernel formulation determines which operators are in the composition and what Origin must check. The structural relationships — functional separation, blind spot geometry, λ-invariance, drift management — are identical.

The structure described above does not require adoption.
It describes relational behavior within **D**.

Interpretation and engagement remain projection-layer decisions.

All statements in this document are bounded over **D**.
No claim is made beyond **D**.
