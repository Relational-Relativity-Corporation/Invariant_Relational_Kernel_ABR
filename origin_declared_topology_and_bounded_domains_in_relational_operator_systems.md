# Origin-Declared Topology and the Necessity of Bounded Domains in Relational Operator Systems

## Abstract

This paper presents a formal argument that the relational operators defined in the ABRCE/ABR framework are well-formed only over explicitly declared, bounded topological domains. We show that when these operators are applied to graphs without declared continuation rules or traversal structure, the operator definitions do not uniquely determine output, and implementations rely on heuristic approximations. Using the ABRCE operator system as a reference implementation, we demonstrate that operator admissibility within the present framework requires a separation between measurement mapping (M : O → D) and operator execution, with topology declared by Origin prior to computation.

---

## 1. Introduction

Modern modeling of complex natural systems—such as magnetospheric dynamics, weather systems, and biological networks—relies heavily on irregular observational data. These observations are typically distributed across non-uniform spatial configurations, such as sensor networks or station arrays.

Standard approaches apply statistical or interpolative methods directly to these irregular structures. These approaches do not declare which relational invariants are preserved under the applied transformations, and when the transformation is non-injective on the edge field, directional or relational distinctions representable in the edge field may not be representable in the output.

This paper addresses a foundational question:

> Under what conditions are the relational operators defined in this framework well-defined and admissible?

---

## 2. The Object Error

Modeling frameworks satisfying conditions (O1)–(O3) of the Object Error (object_error.md §3.1) treat node values as primary and derive relationships secondarily. Under Theorem 1 of that paper, such frameworks cannot distinguish fields with identical marginal distributions but different relational structure.

Under index-local operators (Object Error, Definition 3):

- relational arrangement information is in the null space of the operator class (Theorem 1)
- directional structure representable in the edge field is not representable in the per-entity encoding (Theorem 1b)
- antisymmetric and symmetric components produce identical output under symmetric aggregation (Theorem 3)

We refer to this as the *Object Error*: the condition in which index-local, per-entity representations do not distinguish relational arrangements within the declared domain.

Relational operator systems invert the representational commitment:

> Within the present framework: relations are primary; node-level quantities are derived through declared projection.

This work builds directly on the formal argument presented in **object_error.md**, which establishes the representational constraint for admissible systems under the stated operator definitions.

The present paper does not modify or extend the Object Error itself. Instead, it introduces an additional requirement:

> Even when relations are explicitly represented, evaluation of operators A, B, R, and C as defined in this framework requires declared adjacency relations; without them, the operator definitions do not uniquely determine output.

Within the present framework, resolution of the Object Error is achieved through both:

1. Explicit relational representation (EdgeField)
2. Origin-declared, bounded topology constraining operator evaluation

The Object Error paper (§8.9) establishes that these conditions are sufficient. Whether other structural commitments could also resolve the error class is not excluded.

---

## 3. Kernel Requirements

We consider the relational operator system defined over a domain D with the following operators:

- **A**: NodeField → EdgeField
- **B, R, C**: EdgeField → EdgeField

The operator sequence is:

```
E(x, ρ) = C(R(B(A(x)), ρ))
```

### 3.1 Topological Constraints

Evaluation of operators A, B, and R as defined in this framework requires that the declared topology provide:

- **Deterministic continuation:** for each index, the forward and backward neighbors under the declared topology are uniquely specified by the declaration
- **No undeclared boundary:** every index has a complete neighbor set under the declared topology (no index where continuation is unspecified)
- **Finite indexing:** the index set is finite, consistent with the definition of D

Topologies satisfying these conditions under the present operator definitions include:

- 1D periodic ring
- 2D periodic torus
- 3D periodic torus

These are not the only admissible topologies. Any finite, connected topology with continuation rules satisfying the above conditions — as specified in the declaration — is admissible under the present operator definitions. Periodic topologies satisfy these conditions by construction; non-periodic topologies satisfy them when boundary behavior is explicitly declared as part of the topology specification.

### 3.2 Well-Defined Output and Operator Admissibility

**Definition (Well-defined output).** An operator composition produces *well-defined output* over a declared topology if the operator definitions, together with the topology declaration, uniquely determine the output for every element of D. That is: no undeclared choice is required to compute the output.

**Definition (Operator admissibility).** An operator composition is *admissible* under a declared topology if:

1. The composition produces well-defined output over the declared topology (no undeclared continuation, traversal, or boundary choices are required).
2. The declared representational constraints are satisfied at each stage of the composition (NodeField → EdgeField transition at A; EdgeField → EdgeField through B, R, C; EdgeField → NodeField only through declared Projection).
3. The output remains in D (for V3, guaranteed by C; for V4, guaranteed by the declared projection).

These definitions are used throughout this paper and are referenced in the Object Error paper's discussion of operator validity.

These topologies serve as canonical periodic reference structures satisfying the present operator assumptions. Extension of specific results to topologies with richer connectivity requires separate analysis under the corresponding adjacency structure, as additional adjacency relations may alter spectral behavior, recurrence properties, variance propagation, and the stability conditions of R.

---

## 4. Operator Behavior on Graphs with Undeclared Continuation

Real-world observational data is typically defined on irregular graphs:

- non-uniform spacing
- varying connectivity
- no global symmetry

Applying the operators A, B, and R as defined in this framework directly to such graphs, without declaring continuation rules, means the operator definitions do not uniquely determine output:

- B is defined as the sum of an edge with the next edge along the declared topology; on graphs with varying degree, "next edge" is not uniquely determined without a declared traversal rule
- R is defined over both a forward and backward neighbor edge; on graphs where degree varies by node, these are not uniquely specified without declaration
- A is defined over declared adjacency pairs; on graphs with undeclared connectivity, the edge set itself is not specified

When continuation rules are not declared, implementations supply heuristic choices (e.g., nearest-neighbor selection, cosine similarity, interpolation) where the operator definitions require declared adjacency. These heuristics are not part of the kernel definition. They introduce undeclared choices into the operator pathway, and the relational invariants of the output under heuristic continuation are not formally established.

This does not mean irregular graphs are inadmissible. It means that operator evaluation on irregular graphs requires explicit declaration of:

1. The continuation rule at each index (which neighbors are forward and backward under the declaration)
2. The traversal structure (the complete adjacency specification as declared)
3. Any interpolation or embedding applied within M to produce the declared topology from the irregular observation structure

When these declarations are made, the operators produce well-defined output on the resulting declared topology regardless of its geometric regularity. The admissibility condition is completeness of the declaration, not regularity of the geometry.

---

## 5. Origin Declaration

### 5.1 Definition

We define **Origin** as the entity responsible for declaring the domain D and its topology prior to operator execution.

Formally:

```
M : O → D
```

Where:

- O = observables (raw measurements)
- D = structured domain with declared topology

### 5.2 Role of M

The measurement mapping M is responsible for:

- embedding irregular observations into D under a declared topology
- specifying coordinate systems and units
- performing interpolation where necessary, with declared preserved and discarded invariants

Critically:

> All interpolation and transformation occurs within M, prior to operator evaluation.

Any interpolation, embedding, or topological inference performed within M is a pre-operator step. Its output is a declared topology over which the operator definitions produce well-defined output. The relational invariants of the observable space that are representable in D after M's transformation, and those that are not, are part of the admissibility declaration and must be stated by Origin.

---

## 6. Separation of Concerns

We distinguish three layers:

1. **Observational Layer (O)** — raw measurements, potentially on irregular structures with undeclared continuation
2. **Mapping Layer (M)** — embedding into a structured domain with declared topology, declared coordinate system, and declared preserved/discarded invariants
3. **Operator Layer (ABRCE/ABR)** — relational operator evaluation over the declared topology

This separation ensures that operator evaluation occurs over structures satisfying the admissibility conditions stated in §3.1. Ambiguity in continuation and traversal is resolved within M through declared choices, not within the operator pipeline through undeclared heuristics.

---

## 7. Application: Magnetospheric Modeling

### 7.1 Approach Without Declared Topology

- apply operators directly to station network without declaring continuation rules
- use heuristic continuation (nearest neighbor, interpolation) within the operator pipeline

Result under the present framework:

- operator definitions do not produce uniquely determined output (continuation is not specified)
- relational invariants of the output under heuristic continuation are not established
- representation of operator output as relational structure within a declared domain is not formally supported

### 7.2 Approach With Declared Topology

1. Declare M: map station data to a regular lat/lon grid (2D periodic torus) with declared interpolation method and stated preserved/discarded invariants
2. Declare topology: 2D periodic torus with specified grid resolution
3. Evaluate ABRCE/ABR operators over the declared topology

Under this approach, the operator definitions produce uniquely determined output, and the relational invariants established in the Object Error paper (Theorems 4–6) hold for the output within the declared domain and topology. Operator output is representable as relational structure within the declared topology, under the declared measurement mapping, and with respect to the declared invariants.

The choice of grid resolution, interpolation method, and coordinate system are declared within M and affect which relational distinctions of the original observable space are representable in D. These choices are declared by Origin and are part of the admissibility assessment.

---

## 8. Units and Domain Definition

### 8.1 Problem

Data magnitude affects operator sensitivity. The same physical system measured in different units produces different edge field magnitudes, affecting the behavior of R (through ρ) and C (through the saturation function).

### 8.2 Inadmissible Approaches Under the Present Framework

Transformations that alter pairwise differences between elements of D prior to operator A are inadmissible under the pre-operator transformation constraint (object_error.md §11, operators.rs §11):

- **Normalization** (e.g., min-max scaling, z-score standardization): alters pairwise differences between elements. Inadmissible prior to A unless declared within M with stated preserved and discarded invariants.
- **Statistical scaling** (e.g., variance normalization): couples all elements through a global aggregate prior to relational extraction. Inadmissible prior to A unless declared within M.

These are inadmissible as pre-operator transformations because they alter the relational content — the pairwise differences — that operator A evaluates over. They are not inadmissible in general — they are inadmissible at a specific point in the compositional pathway.

### 8.3 Admissible Approach

Units are part of domain declaration within M:

```
M : O → D_units
```

Example:

- D_Tesla vs. D_nT

This changes the definition of D, not the values within it relative to each other. A uniform scaling T(x) = x / s applied as a declared unit choice within M is admissible because it does not alter pairwise difference ratios — the relational content that operator A evaluates over is invariant under uniform scaling. The scaling is declared as part of M, not applied as a preprocessing step within the operator pipeline.

---

## 9. General Principle

> Within the present framework: operator definitions produce well-defined output over declared structure. Structure must be declared, not inferred, at the point of operator evaluation.

Irregular observational data must be embedded into a domain with declared topology within M before operators A, B, R, and C as defined in this framework are evaluated. The embedding is performed within M, with declared preserved and discarded invariants.

This does not assert that all modeling requires declared topology. It asserts that evaluation of the specific operators defined in this framework requires it for their definitions to produce uniquely determined output.

---

## 10. Implications

This framework applies within domains where observables can be mapped into topologies satisfying the admissibility conditions stated in §3.1.

Domains where this mapping has been demonstrated or is under investigation include:

- geophysical systems (magnetospheric dynamics, weather)
- biological signal processing (cardiac, neural)
- supply chain networks
- machine learning alignment monitoring

This list is illustrative, not exhaustive, and does not constitute a claim that the framework is applicable to all domains. Applicability to a specific domain requires:

1. A declared measurement mapping M : O → D
2. A declared topology satisfying §3.1 conditions
3. A declared coordinate system and unit choice
4. Assessment of which relational invariants of O are preserved under M

Without these declarations:

- operator definitions do not produce uniquely determined output within the present framework
- operator output is not representable as relational structure within a declared D under declared invariants
- relational invariants established in the Object Error paper do not apply to the output

With these declarations:

- operator definitions produce uniquely determined output under the declared topology
- relational invariants (Theorems 4–6) hold for operator output within the declared D
- operator output is representable as relational structure within the declared topology, under the declared measurement mapping, and with respect to the declared invariants

---

## 11. Conclusion

The admissibility of relational operator evaluation within the present framework depends not on the observational data itself, but on the completeness of the declaration specifying the domain into which the data is embedded.

Declaring bounded domains and topology at the level of Origin is not optional within this framework — it is the condition under which the operator definitions produce uniquely determined output.

The Object Error establishes that relational arrangement must be explicitly represented to produce nonzero relational distinctions outside the null space of index-local operators. This work establishes that explicit representation alone is insufficient within the present framework: evaluation of operators A, B, R, and C requires declared adjacency relations, and without that declaration their output is not uniquely determined.

Together, these results define the minimal conditions for admissible relational computation within the present framework:

- relational representation in the edge field (object_error.md)
- origin-declared topology with complete continuation specification (this work)
- operator definitions with stated admissibility conditions (ABRCE/ABR kernel)

Without all three, operator output within this framework is not established as uniquely determined relational structure within D.

---

## References

- object_error.md (Relational representation constraint)
- ABRCE/ABR kernel (operators.rs and associated documentation)
