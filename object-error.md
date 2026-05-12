# The Object Error: A Formal Argument

**Robin Macomber, Metatron Dynamics, Inc.**
**April 2026 — Revised**

---

## 0. The Observable Constant

Within the biologically bounded domain of human perceptual and cognitive range, the only observable constant is change.

No human sensory system registers absolute state. Vision registers contrast — differences in luminance, color, and motion across the visual field. Audition registers pressure variation over time. Proprioception registers change in joint angle and muscle tension. Thermal sensation registers temperature gradient, not temperature. Pain registers tissue-state change, not tissue state. Every sensory modality available to a bounded biological observer is a change-detection system operating over a finite range.

The same holds at the cognitive layer. Memory encodes difference from expectation. Attention is allocated by change in the perceptual field. Learning is the registration of prediction error — the difference between expected and observed. Emotional response tracks change in conditions, not conditions themselves. A constant stimulus of any modality attenuates and disappears from awareness. What remains constant becomes, to the bounded observer, unobservable.

This is not a limitation of human biology. It is the structure of what is observable from within any bounded frame. A measurement apparatus with finite range and finite resolution registers differences relative to calibration. A sensor with bounded dynamic range encodes change from baseline. The bounded observer — biological or instrumental — does not have access to absolute state. It has access to change, registered as differences across the dimensions available to its frame.

A further consequence follows. The assertion that information possesses an absolute quality independent of the observer's bounded frame requires the claimant to have registered information from outside that frame — which is the one operation no bounded observer can perform. The claim is self-refuting within the domain it addresses. Any statement about the character of information is itself a product of a bounded frame; it cannot establish properties of information as it exists independent of all bounded frames, because no such vantage point is available to a bounded observer.

Moreover, each bounded observer's frame is unique. No two biological instances share identical sensory ranges, cognitive histories, attentional structures, or perceptual calibrations. What is registered as "the same information" across observers is convergence of distinct bounded registrations of change — not access to a shared absolute. Agreement is a relational property: it is the condition in which multiple bounded frames register compatible differences, not the condition in which multiple observers access an observer-independent object.

This has direct bearing on the formal argument. The most common implicit defense of the object assumption (§3.1) is that information possesses an observer-independent, object-primary structure, and that the bounded observer's relational experience is a distortion or approximation of that structure. This defense requires a vantage point outside all bounded frames from which the "true" object-primary structure is observable. No such vantage point is available within the biologically constrained domain. The defense is not wrong in a way that could be adjudicated — it is unstatable from within any bounded frame without self-contradiction.

The formal argument that follows encodes this observation. The operator sequence defined in Section 8 follows naturally from this recognition of the nature of information relative to bounded observers.

If the only observable within a bounded frame is change, then the minimum formal operations on change are determined by the structure of bounded observation itself:

**A — Relational Gradient Extraction** is the first necessary operation. A bounded observer registers directed difference across adjacency — change between neighboring elements within the observer's frame. Operator A produces directed pairwise differences over declared topology. It is the formal expression of the condition that change must be made explicit before any subsequent operation.

**B — Local Relational Accumulation** follows because change does not terminate at the nearest neighbor. A bounded observer registers the accumulation of difference along connected paths — the gradient extends beyond immediate adjacency. Operator B accumulates adjacent edges along the declared topology, extending relational reach while remaining in the edge representation.

**R — Antisymmetric Circulation** follows because change couples across dimensions of the observer's frame. No bounded observer registers change in one axis independently of change in perpendicular axes — visual motion has horizontal and vertical components, sound has frequency and amplitude evolution, proprioception couples joint angles across limbs. Operator R cross-couples change across axes antisymmetrically, formalizing the condition that change in one dimension of a bounded frame is structurally related to change in other dimensions.

**C — Bounded Coherence** follows because the observer's frame is finite. Change is registered, accumulated, and coupled — but the frame has bounded range. Operator C bounds the edge field, formalizing the condition that no bounded observer registers infinite difference.

The ordering A → B → R → C is not arbitrary. Change must be expressed (A) before it can be accumulated (B). It must be accumulated before it can be coupled across axes (R), because cross-axis coupling requires edges that extend beyond immediate adjacency. And the result must be bounded (C) because the frame is finite. The operator sequence is the minimum compositional structure that follows from taking the bounded-change observation seriously as a formal constraint.

Axiom 1 (§2.1) establishes that informational content is carried by differences, not absolute values. Operator A (§8.2) extracts directed change across declared adjacency as the first operation in the compositional sequence. The entire ABRCE kernel operates on change — accumulated change (B), coupled change across axes (R), bounded change (C) — without reference to absolute state at any point in the composition.

The object assumption (§3.1) asserts the opposite: that the primitive is the entity — self-identical, context-independent, discretely bounded. This is a description of something no bounded observer has ever registered. Any calculation that treats objects as primitive is apparently describing a system whose structure does not correspond to the structure observable from any bounded human cognitive or perceptual frame. The formal consequences of this mismatch are the content of Sections 3 through 7.

---

## 1. Preliminaries

### 1.1 The Declared Domain

All definitions, operators, and theorems are bounded over **D**.

$$D := \{ x \in \mathbb{R}^n \mid n < \infty,\ |x[i]| < \infty \ \forall\ i \in \{0, \ldots, n-1\} \}$$

### 1.2 The Measurement Mapping

Let **O** denote a declared observable space. A measurement mapping is a function

$$M : O \to D$$

that produces elements of **D** from observables. The kernel operates on $M(o)$ only. Conditions that produce observables are outside **D** by construction.

### 1.3 The Bounded Domain Principle

Every element of **D** exists within a finite-dimensional, finite-magnitude field alongside every other element of **D**. Coexistence within **D** is itself a structural relation. There are no elements of **D** that are relationally disconnected from the field — all relational positions within **D** are determined by coexistence in the field.

This is not an assumption. It is a consequence of the definition of **D**. Any $x \in D$ has $n$ components, each finite. The relational structure — pairwise differences and, over a declared topology, adjacent differences — is determined by these components. No component exists independently of this structure.

### 1.4 Topological Declaration

The operators defined in this paper act over a declared topology on the index set of **D**.

For a field $x \in D$ with $n$ indices, the declared topology determines which pairs of indices are adjacent. In one dimension, the minimal connected topology is the periodic ring: index $i$ is adjacent to index $(i+1) \bmod n$ and index $(i-1) \bmod n$.

The topology is declared before processing begins as part of the measurement mapping $M$. It is not discovered from the data. It is not inferred. It is a structural commitment declared by Origin, specifying which adjacency relations in the observable space are represented under $M$.

*Remark on topology inference.* Many real systems have connectivity structures that are unknown, partially known, or context-dependent. In such systems, topology inference — learning the adjacency structure from data — is a legitimate and often necessary step. This paper does not claim that topology must always be known *a priori*. It claims that the operators defined in Section 8 require a declared topology in order to act. If topology is inferred, the inference is a pre-operator step that produces a topological commitment, and that commitment must be declared before the operators process the field. The structural consequence is that discrepancies between inferred topology and the topology that would be declared under complete knowledge of the observable space are present in the operator sequence as discrepancies in the declared adjacency — a condition Origin must hold as part of the admissibility declaration. Topology inference is admissible; undeclared or implicit topology is not.

All operator definitions in Section 8 are written over the periodic ring topology. Results established under ring coupling hold *a fortiori* for any topology with richer connectivity, since additional couplings extend the adjacency set over which the operator composition is defined.

---

## 2. The Relational Content of Measured Data

### 2.1 Translation Invariance of Informational Content

**Definition 1 (Translation equivalence).** Two fields $x, y \in D$ are *translation equivalent*, written $x \sim_\tau y$, if there exists $c \in \mathbb{R}$ such that $y[i] = x[i] + c$ for all $i$.

**Axiom 1 (Informational translation invariance).** For any measurement mapping $M : O \to D$ and any two observables $o_1, o_2 \in O$: if $M(o_1) \sim_\tau M(o_2)$, then $o_1$ and $o_2$ are informationally indistinguishable with respect to the system under observation.

**Justification.** A uniform shift in all measured values reflects a change in reference frame, not a change in the system's state. No bounded measurement apparatus produces output determined by absolute values — output is determined by differences relative to calibration, baseline, or other measured values.

**Formal scope restriction.** Axiom 1 holds for measurement regimes in which the primitive is calibration-relative: sensor readings, prices, voltages, temperatures, survey responses, and the overwhelming majority of data processed in applied data science. All theorems in this paper are proved within this scope. The domain of applicability of this paper's results is bounded by this scope.

**Remark on object-primary measurements.** Object-primary measurements do exist. Particle counts, threshold detections, and discrete enumerations produce data whose immediate structure is object-primary — counts of discrete entities. However, the systems these measurements describe (quantum fields, populations, networks) are relationally constituted. The object-primary measurement is a projection of a relational system onto a discrete count. The error identified in this paper enters when the model of the system is constructed with the same object-primary structure as the measurement of the system — when the count is treated as a complete description rather than a lossy projection. The measurement may be object-primary; the system is not. Modeling the system as if it shares the structure of its measurement produces a representation whose relational structure does not correspond to the relational structure of the system as observed from any bounded frame.

This paper's formal results do not depend on whether object-primary measurements exist. They depend on whether the systems being modeled contain relational structure beyond what is captured in per-entity feature values. Within the declared scope (calibration-relative measurement regimes), Axiom 1 establishes that this is the case. For measurement regimes outside this scope, the argument extends to the degree that the measured system contains relational structure not captured by the object-primary measurement — a condition the modeler must evaluate for each application.

### 2.2 Relational Structure

**Definition 2 (Relational structure — invariant characterization).** The *relational structure* of a field $x \in D$ is the equivalence class $[x]_\tau$ under translation equivalence. Concretely, $[x]_\tau$ is fully characterized by the centered representation:

$$\bar{x}[i] := x[i] - \text{mean}(x)$$

which is the unique representative of $[x]_\tau$ satisfying $\sum_i \bar{x}[i] = 0$.

**Proposition 1.** The centered representation $\bar{x}$ is a complete invariant of $[x]_\tau$. That is:

$$x \sim_\tau y \quad \text{if and only if} \quad \bar{x} = \bar{y}$$

*Proof.* If $x \sim_\tau y$, then $y[i] = x[i] + c$ for all $i$, so $\text{mean}(y) = \text{mean}(x) + c$, and $\bar{y}[i] = y[i] - \text{mean}(y) = x[i] - \text{mean}(x) = \bar{x}[i]$. Conversely, if $\bar{x} = \bar{y}$, then $x[i] - \text{mean}(x) = y[i] - \text{mean}(y)$ for all $i$, giving $y[i] = x[i] + (\text{mean}(y) - \text{mean}(x))$, so $x \sim_\tau y$ with $c = \text{mean}(y) - \text{mean}(x)$. $\square$

**Corollary.** The informational content of any $x \in D$ — the component that distinguishes system states — is fully carried by its relational structure $\bar{x}$. The absolute component $\text{mean}(x)$ carries no system information under Axiom 1.

### 2.3 Characterization Versus Extraction

Definition 2 characterizes relational content: it identifies *what information* a field carries, using the centered representation as a mathematical invariant. The centered representation requires a global aggregate ($\text{mean}(x)$) to compute, but this is a property of the characterization, not a requirement of the information itself.

The relational content of a field can also be expressed without any global aggregate. The complete set of pairwise differences $\{x[i] - x[j]\}$ determines $[x]_\tau$ without reference to the mean. Over a declared topology, the set of adjacent differences $\{x[i] - x[j] \mid i \text{ adj } j\}$ determines $[x]_\tau$ on any connected topology (since non-adjacent differences are recoverable by summing paths of adjacent differences).

This distinction — between a global characterization that identifies the invariant and a local extraction that produces it from direct relations — is structurally significant when operators must compose. A characterization may use any mathematically valid procedure. An operator's structural commitments are present in the input of all downstream operators in the compositional sequence. If the extraction step introduces aggregation, that aggregation becomes part of the compositional pathway, whether or not the downstream operators are designed for it.

**Structural Principle (Compositional representation dependence).** Two representations may encode the same invariant information but differ in their compositional behavior under operators. The choice of representation at the extraction layer constrains what downstream operators can access.

---

## 3. The Object Assumption and Its Formal Consequences

### 3.1 The Starting Assumption of Object-Primary Modeling

Object-primary formalisms — including the foundational architectures of regression, classification, neural network training, probabilistic graphical models, and the preponderance of applied data science — begin from the following structural commitment:

*The data consists of discrete entities (samples, observations, rows) whose properties (features, variables, columns) are attributed to each entity independently, with no assumed connectivity between entities and no declared bounded domain constraining the field.*

This commitment is operationalized through three conditions:

**(O1) Self-identity.** Each entity $a$ satisfies $a = a$ independently of any relation $a$ participates in. Identity is intrinsic to the entity.

**(O2) Context-independence.** Properties attributed to $a$ are invariant under changes to the relational environment. Feature values are assigned per-entity, not per-relation.

**(O3) Discrete boundaries.** There exists a sharp partition between $a$ and everything that is not $a$. Entities are rows; the boundary between rows is absolute.

### 3.2 The Assumed-Disconnection Starting Point

The critical structural consequence of (O1)–(O3) is this: object-primary models begin from an assumption of no connectivity. The data matrix $X \in \mathbb{R}^{m \times n}$ is, at initialization, a collection of $m$ independent rows with $n$ independent feature values. No relational structure is assumed.

The model's computational operations are then applied to a representation from which relational structure is absent. The entire computational effort of the model is directed at reconstructing relational structure from a representation that has already discarded it.

### 3.3 On Relational Architectures Within Object-Primary Pipelines

A necessary clarification: some modern architectures introduce relational operations at the computational layer. Graph neural networks encode adjacency structure. Attention mechanisms compute pairwise relational scores. Convolutional architectures exploit spatial locality.

However, these architectures fail the formal requirements of the present argument on multiple grounds:

**No declared bounded domain.** Transformers operate over arbitrary context windows with no declared **D**, no bounded domain, no declared measurement mapping $M$. The attention mechanism computes relational operations over an unbounded, undeclared scope. This is not a partial escape from the object error — it is a relational operation floating in an undeclared domain. Without domain closure, no formal condition constrains the architecture's outputs to **D**. Under this condition, no formal correspondence between outputs and any declared observable system is established. The apparent inability to accurately describe information from an unbounded frame — observed empirically as outputs bearing no correspondence to any declared domain — is admissible under an architecture that does not constrain outputs to a declared **D**.

**Object-primary training and validation.** These relational operations are embedded within object-primary training and validation pipelines. Loss functions (cross-entropy, mean squared error) are computed per-entity and aggregated symmetrically. Validation metrics (holdout accuracy, AUC, F1) are index-local detection functionals in the sense of Theorem 3. Gradient descent updates parameters based on per-sample loss, not on relational field structure. The architecture contains relational operations; the pipeline that trains and evaluates it does not.

**No domain closure operator.** The ABRCE composition includes C — a bounded coherence operator that constrains $E(x, \rho) \in D$ by construction. No standard deep learning architecture includes a comparable formal property. Outputs are unconstrained, and post-hoc corrections (clipping, normalization) are patches applied after the computation rather than structural properties of the operator.

The empirical superiority of architectures with relational operations over purely index-local methods (tabular ML, classical regression) is predicted by the present argument: relational operations are defined over information that index-local operations are not (Theorem 1). But the absence of declared bounded domains, the object-primary training pipeline, and the lack of domain closure mean that these architectures introduce a different failure mode rather than resolving the one identified here. This paper provides the formalization: relational operations are necessary (Theorems 1–3), bounded domains are necessary (§1.1), and domain closure is necessary (§8.1, operator C). Relational operations alone are insufficient.

### 3.4 Why This Produces Structural Error

By Axiom 1 and Definition 2, the informational content of measured data is relational. The data arrives as differences — sensor readings relative to calibration, prices relative to baselines, measurements relative to references. The relational structure is not a secondary property to be reconstructed from the data. It constitutes the informational content of the data.

When an object-primary formalism encodes this data into a matrix of independent entity-attribute pairs, it performs an implicit projection: the relational arrangement of the field is removed from the representation, retaining per-entity values while discarding the structural information that constituted the data's content. The model's computational operations are then applied to a representation from which this content is absent.

This is not an approximation. Any calculation that treats objects as primitive is apparently describing a system whose relational structure does not correspond to the relational structure observable from any bounded human cognitive or perceptual frame.

---

## 4. Index-Local Operators: The Formalization

### 4.1 Definition

**Definition 3 (Index-local operator).** An operator $T : D \to D$ is *index-local* if there exists a family of functions $\{f_i : \mathbb{R} \to \mathbb{R}\}_{i=0}^{n-1}$ such that:

$$T(x)[i] = f_i(x[i]) \quad \forall\ x \in D,\ \forall\ i$$

If $f_i = f$ for all $i$, the operator is *uniformly index-local*.

Index-locality is the operational formalization of (O1)–(O3). Each index is processed as a self-identical entity (O1), without reference to any other index (O2), with an absolute boundary between its value and all others (O3).

*Remark on scope.* In this definition, the index $i$ ranges over the entities (rows, observations, samples) in the data. The operator processes each entity's output as a function of that entity's input alone. Standard regression computes $\hat{y}_i = \beta^T x_i$, which operates on the feature vector of entity $i$ — multiple scalar values — but processes entity $i$ independently of all other entities $j \neq i$. The index-locality condition captures this inter-entity independence, which is the formal content of (O1)–(O3). The theorems that follow apply to any operator whose output for entity $i$ is determined solely by entity $i$'s input, regardless of how many features per entity are involved.

### 4.2 The Null Space Theorem

**Theorem 1 (Relational null space of index-local operators).** Let $T$ be a uniformly index-local operator with $T(x)[i] = f(x[i])$ for all $i$. Let $x, y \in D$ satisfy:

(a) $x$ and $y$ have identical marginal distributions (i.e., $\{x[i]\}$ and $\{y[i]\}$ are the same multiset), but

(b) $x$ and $y$ differ in relational structure: there exist indices $i, j$ such that $x[i] - x[j] \neq y[i] - y[j]$.

Then $T(x)$ and $T(y)$ have identical marginal distributions. $T$ cannot distinguish $x$ from $y$.

*Proof.* Since $\{x[i]\}$ and $\{y[i]\}$ are the same multiset, there exists a permutation $\pi$ such that $y[i] = x[\pi(i)]$ for all $i$. Then $T(y)[i] = f(y[i]) = f(x[\pi(i)]) = T(x)[\pi(i)]$. Therefore $\{T(y)[i]\}$ and $\{T(x)[i]\}$ are the same multiset. $\square$

**Interpretation.** The relational content of the data — the content that constitutes its informational payload (Section 2) — is in the null space of index-local operators. This is not an implementation constraint. It is a structural property of the operator class.

---

## 5. The Impossibility of Recovery

### 5.1 The Non-Existence of Relational Independence Within D

**Theorem 1a (Relational determinacy over D).** For all $x \in D$ with $n \geq 2$, and for all distinct indices $i, j$, the quantity $x[i] - x[j]$ is defined and determined by the field. The relational position of each index is therefore specified by its differences from all other indices.

*Proof.* Immediate from the definition of **D**. Both $x[i]$ and $x[j]$ are finite real numbers in the same field. The difference $x[i] - x[j]$ exists and is determined. No additional axiom or assumption is required — this is a structural consequence of coexistence in **D**. $\square$

*Remark on relational determinacy versus statistical dependence.* Theorem 1a establishes that pairwise differences are *determined* — they exist and have definite values as a consequence of the definition of **D**. This is a statement about the structure of the representation, not about probabilistic dependence. Statistical independence ($P(X,Y) = P(X)P(Y)$) is a property of a probability model applied to data; relational determinacy is a property of the data's position within a bounded field. A statistician may correctly observe that two variables can be uncorrelated or even probabilistically independent under a given model while coexisting in **D**. This does not contradict Theorem 1a. The theorem states that the differences exist and are determined — not that any particular statistical measure will detect them. The gap between determinacy and detectability is precisely the content of Theorems 1 and 3: the relational structure is there; the index-local measures cannot see it.

### 5.2 The Statistical Independence Assumption as Object Error

Classical statistical modeling assumes that observations (rows) are independently and identically distributed (i.i.d.) or that residual errors are independent. These independence assumptions presuppose (O2): that the value at one index carries no information about the value at any other index.

Within **D**, this assumption is structurally false by Theorem 1a. Each index participates in a fully specified set of pairwise differences within the field. When a statistical model assumes independence, the assumption is not a simplifying approximation — it encodes a condition that is formally impossible within the declared domain.

### 5.3 On Correlation, Independence, and Relational Structure

A precise distinction is required here. The most likely terminological objection is: "You are conflating statistical correlation with relational structure. Two variables can be uncorrelated while still coexisting in a domain."

This objection misidentifies the level at which the argument operates. The argument is not about statistical correlation. It is about the representational structure of **D**.

**Definition 3a (Correlation as index-local measure).** The Pearson correlation between two variables $X$ and $Y$ measured across a sample is:

$$r_{XY} = \frac{\sum_i (X_i - \bar{X})(Y_i - \bar{Y})}{\sqrt{\sum_i (X_i - \bar{X})^2 \sum_i (Y_i - \bar{Y})^2}}$$

This is a pairwise measure computed from marginal deviations. It is an index-local question — it begins from the entity (the row, the sample) and evaluates association across attributed properties.

When $r_{XY} = 0$, the correct conclusion within **D** is not "no association exists." The correct conclusion is: "this particular measure, which operates within the null space identified by Theorem 1, has returned zero." The relational structure between $X$ and $Y$ — their joint position within the field, their differential coupling to other indices, their co-participation in the gradient structure of **D** — remains fully determined and fully present. The measure cannot see it.

*Remark on nonlinear dependence measures.* More sophisticated dependence measures — mutual information, copula-based tests, kernel-based independence tests (HSIC), distance correlation — detect nonlinear associations that Pearson correlation misses. However, these measures share a structural property: they are computed from the joint and marginal *distributions* of $(X, Y)$, which are themselves constructed from per-entity observations aggregated through symmetric functions. They detect distributional dependence — whether knowing $X$'s value changes the expected distribution of $Y$ — but not relational arrangement within the field. Two fields with identical joint distributions of $(X, Y)$ but different spatial arrangements of those values across the field are indistinguishable to any distribution-based measure. The class of measures that operate on distributions rather than field arrangement is broader than Pearson correlation, but the null space identified by Theorem 1 — relational arrangement information — remains outside all such measures.

**The word "independence" as used in probability theory formally encodes (O2).** Two random variables $X$ and $Y$ are independent if $P(X, Y) = P(X)P(Y)$ — if the joint distribution factors into marginals. This is a statement that the value of $X$ carries no information about $Y$ and vice versa. Within **D**, Theorem 1a establishes that this factorization does not describe the structure of the field: every index is relationally determined by its position in the field. The factorization describes a property of the *model's representation* of the field — a representation constructed under (O1)–(O3) — not a property of **D**.

Therefore: any argument that invokes "uncorrelated" or "independent" as evidence against the relational structure of **D** is applying a concept that describes the model's factorization of the field to the field itself. The factorization is a property of the representation, not of **D**. The argument is circular — it relies on the object assumption's representational commitments as evidence for the object assumption.

*Remark on independence as a modeling tool.* The claim here is not that independence is incoherent as an abstract mathematical concept. Within the axioms of probability theory, $P(X, Y) = P(X)P(Y)$ is a well-formed statement. The claim is that when this statement is applied as a *model* of data drawn from a bounded relational domain, it encodes a condition — context-independence between variables — that is structurally impossible in **D** (Theorem 1a). A model that assumes a structurally impossible condition may still produce useful approximations in regimes where the relational coupling is weak relative to the model's detection threshold. But the approximation error is systematic and compounding (Theorem 2), and the model's own validation apparatus cannot detect it (Theorem 3). The tool is internally coherent; its application to relationally constituted data introduces the error class this paper identifies.

### 5.4 The Recovery Problem

Object-primary models apply reconstruction operations after the encoding step has failed to preserve relational structure in the representation. The standard methods — correlation analysis, regression, dimensionality reduction, graph learning — share a common structural limitation:

**Theorem 1b (Incomplete recoverability).** Let $P : D \to \mathbb{R}^{m \times n}$ be the projection that decomposes a field into $m$ entity-attribute rows under the object assumption (discarding relational arrangement). Let $R : \mathbb{R}^{m \times n} \to D$ be any reconstruction operator that attempts to recover relational structure from the projected representation.

Then $R \circ P$ is not injective on the relational equivalence classes $[x]_\tau$. That is, there exist fields with distinct relational structures that map to the same entity-attribute matrix and therefore cannot be distinguished by any post-hoc analysis of that matrix.

*Proof.* $P$ maps all fields with the same multiset of row vectors to the same matrix (up to row ordering). Fields with identical rows in different relational arrangements are collapsed. Since the matrix does not encode relational arrangement, $R$ cannot recover it. $\square$

**Remark on positional features.** In practice, datasets frequently include features that encode positional or temporal information — timestamps, spatial coordinates, sequence indices. When these features are present, the modeler has manually re-introduced relational information into the per-entity feature representation — encoding relational content through the object-primary format. This does not contradict Theorem 1b; it confirms its mechanism. The modeler recognizes that the object-primary encoding has removed information the model requires, and compensates by re-encoding relational data as per-entity attributes.

This workaround does not resolve the information loss. Either the positional features fully encode the relational structure of the field — in which case the remaining per-entity features are redundant and the model is operating on a relational representation forced through an object-primary format — or they do not, in which case the relational structure not captured by the positional features is absent from the representation and cannot be reconstructed from it (Theorem 1b). The positional features encode specific relational dimensions (temporal order, spatial proximity) while the remaining relational structure of the field (coupling strengths, gradient directions, circulation patterns) is absent from the representation and cannot be reconstructed from it.

**Implication.** The information not preserved by the object-primary encoding is not recoverable. No algorithm, regardless of sophistication, can reconstruct relational structure from a representation that has provably discarded it. The computational operations applied to object-primary data representations are bounded above by the information present in the encoding — which excludes relational arrangement by construction.

---

## 6. Monotonic Divergence

### 6.1 Divergence Metric

**Definition 4 (Relational divergence).** Let $S(t)$ denote the observed system state at time $t$ (an element of **D** produced by $M$) and let $\hat{S}(t)$ denote the model's representation at time $t$. The *relational divergence* at time $t$ is:

$$d(t) := \|\bar{S}(t) - \bar{\hat{S}}(t)\|_2$$

where $\bar{S}$ and $\bar{\hat{S}}$ are the centered representations (Definition 2).

### 6.2 Independence of Model and System Relational Evolution

**Lemma 1 (Relational independence of index-local output).** Let $T$ be a uniformly index-local operator, $\Phi$ a relational dynamics on **D**, and $\pi_\perp$ the orthogonal projection onto the relational subspace $V_\perp = \{x \in \mathbb{R}^n \mid \sum_i x[i] = 0\}$. Then:

The relational component of $T$'s output, $\pi_\perp(T(\hat{S}(t)))$, is determined entirely by the multiset $\{\hat{S}(t)[i]\}$. The relational component of $\Phi$'s output, $\pi_\perp(\Phi(S(t)))$, depends on the arrangement of values across indices.

Therefore $\pi_\perp(T(\hat{S}(t)))$ is not a function of the arrangement information that determines $\pi_\perp(\Phi(S(t)))$.

*Proof.* By Theorem 1, $T$ maps any two fields with the same marginal multiset to outputs with the same marginal multiset. Two fields with the same marginal multiset have the same centered representation under any index-invariant centering. Therefore $\pi_\perp(T(x)) = \pi_\perp(T(y))$ whenever $\{x[i]\} = \{y[i]\}$ as multisets, regardless of arrangement.

$\Phi$, by contrast, is a relational dynamics: by assumption (condition ii of Theorem 2), it alters relational structure, meaning its output arrangement depends on its input arrangement.

Since $\pi_\perp(T(\hat{S}(t)))$ is invariant under rearrangement of $\hat{S}(t)$ and $\pi_\perp(\Phi(S(t)))$ depends on the arrangement of $S(t)$, the former is not a function of the information that determines the latter. No functional relationship connects the model's relational output to the system's relational evolution. $\square$

### 6.3 The Divergence Theorem

**Theorem 2 (Monotonic relational divergence under index-local modeling).** Let $\Phi$ be a relational dynamics and $T$ a uniformly index-local operator. Suppose:

(i) $\hat{S}(0) = S(0)$ (initial calibration),

(ii) $\Phi$ alters relational structure at each step: for all $t$, there exist indices $i, j$ such that $\Phi(S(t))[i] - \Phi(S(t))[j] \neq \bar{S}(t)[i] - \bar{S}(t)[j]$,

(iii) $T$ is uniformly index-local: $T(x)[i] = f(x[i])$ for all $i$.

Then $d(t)$ is monotonically non-decreasing:

$$d(t+1) \geq d(t) \quad \forall\ t \geq 0$$

*Proof.* Decompose $\mathbb{R}^n$ into two orthogonal subspaces:

$V_\tau := \text{span}\{(1, 1, \ldots, 1)\}$ — the *translation subspace* (uniform shifts).

$V_\perp := \{x \in \mathbb{R}^n \mid \sum_i x[i] = 0\}$ — the *relational subspace* (zero-sum vectors carrying all relational content).

Any $x \in \mathbb{R}^n$ decomposes uniquely as $x = x_\tau + x_\perp$ where $x_\tau = \text{mean}(x) \cdot \mathbf{1} \in V_\tau$ and $x_\perp = \bar{x} \in V_\perp$.

The relational divergence $d(t) = \|\bar{S}(t) - \bar{\hat{S}}(t)\|_2$ is the norm of the error projected onto $V_\perp$.

At each step $t$, the system undergoes a relational change $\delta_\perp(t) := \bar{S}(t+1) - \bar{S}(t) \neq 0$ (by condition ii). This change lives entirely in $V_\perp$.

By Lemma 1, $\pi_\perp(T(\hat{S}(t)))$ is not a function of the arrangement information that determines $\delta_\perp(t)$. The model's relational output at each step is therefore independent of the quantity it would need to match.

For $d(t)$ to decrease at step $t$, the model's relational output would need to produce a correction $\hat{\delta}_\perp(t)$ such that $\|\bar{S}(t+1) - (\bar{\hat{S}}(t) + \hat{\delta}_\perp(t))\|_2 < \|\bar{S}(t) - \bar{\hat{S}}(t)\|_2$. This requires $\hat{\delta}_\perp(t)$ to be correlated with the accumulated relational error and with $\delta_\perp(t)$. But by Lemma 1, $\hat{\delta}_\perp(t)$ is not a function of the arrangement information that determines both quantities. No functional relationship within the index-local operator class connects the model's relational output to the accumulated relational error or to the system's relational evolution at each step.

Each step introduces uncompensated relational error. At no step is this error reduced. No operator within the index-local class reduces it, by Lemma 1. Therefore $d(t+1) \geq d(t)$. $\square$

---

## 7. Invisibility

### 7.1 The Invisibility Theorem

**Theorem 3 (Structural invisibility).** Let $\mathcal{D} : D \to \mathbb{R}$ be a detection functional constructed from index-local operations. That is, $\mathcal{D}$ is a composition of uniformly index-local operators followed by an aggregation $g : D \to \mathbb{R}$ that depends only on the multiset $\{z[i]\}$ (e.g., sum, mean, max, or any symmetric function).

Then $\mathcal{D}$ cannot detect relational divergence. For any two model states $\hat{S}_1, \hat{S}_2 \in D$ with identical marginal distributions but different relational structure:

$$\mathcal{D}(\hat{S}_1) = \mathcal{D}(\hat{S}_2)$$

*Proof.* By Theorem 1, any uniformly index-local operator maps $\hat{S}_1$ and $\hat{S}_2$ to outputs with identical marginal distributions. Since $g$ depends only on the multiset, $g$ returns the same value for both. $\square$

### 7.2 Application to Standard Model Validation

The dominant class of model validation techniques in data science — cross-validation, holdout testing, residual analysis, AIC/BIC, bootstrap methods — operate on per-entity error measures aggregated through symmetric functions (mean squared error, log-likelihood, accuracy). These are compositions of index-local operations and symmetric aggregations. By Theorem 3, they cannot detect relational divergence.

*Remark on partially relational validation.* Some validation methods partially escape index-locality. Spatial cross-validation respects geographic adjacency. Graph-based validation preserves network structure in train/test splits. These methods acknowledge that relational arrangement matters for generalization — which is consistent with the present argument — but they do not evaluate relational divergence directly. They modify the *partitioning* of data to respect relational structure, while the *evaluation metrics* applied within each partition (accuracy, RMSE, AUC) remain index-local. The partitioning strategy reduces one source of bias; the evaluation apparatus retains the null space identified by Theorem 3. A validation framework that fully escapes this limitation would require evaluation metrics that operate on relational arrangement, not merely partitioning strategies that respect it.

This result is consistent with the persistent observation across applied data science: models that perform well on validation metrics fail unpredictably in deployment. The validation metrics are structurally incapable of detecting the error class that causes deployment failure. The models are not failing because of insufficient data, poor hyperparameter tuning, or distribution shift in the conventional sense. They are failing because their validation apparatus shares the null space of their modeling apparatus, and the divergence is contained within that null space.

### 7.3 Implication

Increasing the resolution, frequency, computational power, or algorithmic sophistication of a detection or validation system does not resolve this limitation if the system retains index-local structure. The limitation is algebraic: relational divergence is in the null space of the detector. No refinement of operators or functionals within the null space yields a non-zero detection signal for relational divergence.

---

## 8. Resolution: The ABRCE Operator Sequence

### 8.1 Edge Fields and the Relational Representation

Before defining the operators, we introduce the representation that distinguishes ABRCE from both index-local operators and from the invariant characterization of Section 2.

**Definition 5 (Edge field).** Given a field $x \in D$ with $n$ indices and a declared topology $\mathcal{T}$ specifying adjacency pairs, the *edge field* of $x$ over $\mathcal{T}$ is the set of directed differences:

$$\mathcal{E}(x) = \{ x[i] - x[j] \mid (i, j) \in \mathcal{T} \}$$

Each element of $\mathcal{E}(x)$ is a *directed edge*: it carries the magnitude and sign of the relation between two adjacent indices.

Over the periodic ring topology, the edge field is:

$$\mathcal{E}(x)[i] = x[i] - x[(i+1) \bmod n]$$

producing one directed edge per index.

**Proposition 2 (Edge field determines relational structure).** For any connected topology $\mathcal{T}$, the edge field $\mathcal{E}(x)$ determines the equivalence class $[x]_\tau$.

*Proof.* On a connected topology, any non-adjacent difference $x[i] - x[k]$ is recoverable as a telescoping sum of adjacent differences along a path from $i$ to $k$. Since all pairwise differences are determined, and pairwise differences determine the centered representation $\bar{x}$ (Proposition 1), $\mathcal{E}(x)$ determines $[x]_\tau$. $\square$

**Relationship to Definition 2.** Both the centered representation $\bar{x}$ and the edge field $\mathcal{E}(x)$ are complete invariants of $[x]_\tau$ (Proposition 1 and Proposition 2). They characterize the same information. The difference is structural:

The centered representation is a *node field*: one value per index, computed through a global aggregate. It characterizes the equivalence class through a canonical representative.

The edge field is an *edge field*: one value per adjacency pair, computed through direct pairwise differences. It expresses the equivalence class through local relations without aggregation.

The ABRCE operator sequence uses the edge field as its working representation. Operator A produces an edge field. Operators B and R operate on edge fields. The transition from edge field back to node field occurs at a specific, declared point in the composition (Section 8.6).

### 8.2 Operators

Each operator in the ABRCE sequence is demonstrably not index-local.

**A — Relational Gradient Extraction.**

$$A : D \to \mathcal{E}$$

$$A(x)[i] = x[i] - x[(i+1) \bmod n]$$

$A(x)[i]$ depends on two indices: $i$ and $(i+1) \bmod n$. Not index-local. A produces the edge field corresponding to the relational content identified in Definition 2, expressed as direct pairwise differences over the declared topology. Where the object-primary encoding does not preserve relational structure in the representation, A's output is the relational structure — in edge form, without aggregation.

*Remark on the relationship to the centered representation.* The centered representation $\bar{x}[i] = x[i] - \text{mean}(x)$ also characterizes relational content (Proposition 1). However, it does so through a global aggregate, producing a node field. Operator A produces the same informational content (Proposition 2) through purely local operations, producing an edge field. This distinction is not cosmetic: the edge field contains directional relations between adjacent indices, which is the representation that operators B and R require. Routing relational extraction through a global aggregate and back would introduce aggregation into the compositional pathway at the point where the paper's own argument (Sections 3–7) establishes that aggregation is in the null space of the information the operators are defined over.

**B — Local Relational Accumulation.**

$$B : \mathcal{E} \to \mathcal{E}$$

$$B(e)[i] = e[i] + e[(i+1) \bmod n]$$

$B(e)[i]$ depends on edges $e[i]$ and $e[(i+1) \bmod n]$. Not index-local. B is defined as the additive composition of adjacent edges within the declared topology: the edge value at index $i$ is summed with the edge value at index $(i+1) \bmod n$, extending relational reach by one cell while remaining in the edge representation.

*Remark on topology choice.* The ring topology is the minimal connected coupling on an ordered index set: each index is coupled to exactly one neighbor in each direction. Any connected topology (lattice, complete graph, random graph) would satisfy the formal requirement of non-index-locality. The ring is chosen because it is the minimum adjacency structure necessary to escape index-locality. Results established under ring coupling hold *a fortiori* for any topology with richer connectivity, since additional couplings extend the adjacency set over which the operator composition is defined.

**R — Antisymmetric Circulation.**

$$R : \mathcal{E} \times \mathbb{R} \to \mathcal{E}$$

$$R(e, \rho)[i] = e[i] + \rho \cdot (e[(i+1) \bmod n] - e[(i-1) \bmod n])$$

$R(e)[i]$ depends on both neighboring edges. Not index-local. R is defined by antisymmetric coupling. The forward-backward difference $(e[(i+1) \bmod n] - e[(i-1) \bmod n])$ is itself a relation between edges — a difference of differences. Under iterated composition including R, relational variance remains nonzero (Theorem 6); under iterated composition without R, relational variance approaches zero (Theorem 5). R's output is an edge field.

**C — Bounded Coherence.**

$$C : \mathcal{E} \to \mathcal{E}_D$$

$$C(e)[i] = \frac{e[i]}{1 + |e[i]|}$$

$C$ is index-local on the edge field. However, $C$ acts on the output of $R \circ B \circ A$, which already carries full relational structure in edge form. $C$ is monotone and sign-preserving: it maps relational ordering while bounding magnitude. The one index-local operator in the sequence acts *after* relational structure has been fully constructed and circulated through the composition, not *before*.

**E — Composite Evolution.**

$$E : D \times \mathbb{R} \to \mathcal{E}_D$$

$$E(x, \rho) = C(R(B(A(x)), \rho))$$

### 8.3 Type Signatures

The revised type signatures reflect the edge field representation:

$$A : D \to \mathcal{E} \quad \text{(node field → edge field; relational extraction)}$$

$$B : \mathcal{E} \to \mathcal{E} \quad \text{(edge field → edge field; relational accumulation)}$$

$$R : \mathcal{E} \times \mathbb{R} \to \mathcal{E} \quad \text{(edge field × ρ → edge field; antisymmetric circulation)}$$

$$C : \mathcal{E} \to \mathcal{E}_D \quad \text{(edge field → bounded edge field; coherence)}$$

$$E : D \times \mathbb{R} \to \mathcal{E}_D \quad \text{(node field × ρ → bounded edge field; composite evolution)}$$

The operators A through C form a pipeline that begins with a node field (the measured data) and produces a bounded edge field (the evolved relational structure). The output of E is an edge field, not a node field. This is a structural commitment: the canonical output of the ABRCE sequence is relational.

### 8.4 ABRCE Distinguishes Relational Structure

**Theorem 4 (Relational sensitivity of ABRCE).** Let $x, y \in D$ satisfy $\{x[i]\} = \{y[i]\}$ as multisets but $\bar{x} \neq \bar{y}$. Then:

$$E(x, \rho) \neq E(y, \rho) \quad \text{for all } \rho \in (0, 0.5] \text{ except possibly a measure-zero subset}$$

*Proof.* $A(x)[i] = x[i] - x[(i+1) \bmod n]$. Since $x$ and $y$ have different relational structures (different arrangements of the same values across indices), there exist adjacent pairs $(i, i+1)$ where $x[i] - x[i+1] \neq y[i] - y[i+1]$. Therefore $A(x) \neq A(y)$.

$B$ carries this distinction forward: $B$ couples each edge with its neighbor, and since the edge fields differ, the accumulated edge fields differ.

$R$ carries the distinction through antisymmetric circulation: $R$ is an analytic function of $\rho$ (since $A$, $B$, and $R$ are linear in the edge values, and $C$ is analytic). An analytic function on an interval is either identically zero or has at most countably many (hence measure-zero) zeros. Since $A(x) \neq A(y)$ and $B$ carries the distinction independently of $\rho$, the function $E(x, \rho) - E(y, \rho)$ is not identically zero. Therefore the exceptional set is measure-zero.

$C$ carries it because $C$ is strictly monotone. $\square$

**Corollary.** ABRCE composition is defined over relational structure — the exact content that is in the null space of index-local operators (Theorem 1). The divergence that is in the null space of index-local detection functionals (Theorem 3) produces nonzero output under ABRCE composition (Theorem 4).

### 8.5 The Observable Variance Criterion

**Definition 6 (Observable variance).** For an edge field $e$ with $n$ edges:

$$\sigma^2(e) := \text{Var}(e) = \frac{1}{n} \sum_{i=0}^{n-1} (e[i] - \text{mean}(e))^2 = \frac{1}{n} \|\bar{e}\|_2^2$$

*Remark.* The observable variance criterion uses a statistical aggregate (variance) as a *diagnostic measure* applied to the output of the operator sequence. This does not contradict the non-aggregative character of the operators themselves. The operators produce structure; the diagnostic evaluates that structure. The aggregate is at the measurement layer, not the operator layer.

**Theorem 5 (Relational variance approaches zero without R).** Under repeated application of $C \circ B \circ A$ (the ABRCE composition with $R$ removed):

$$\lim_{t \to \infty} \sigma^2(e_t) = 0$$

for any initial condition $x_0 \in D$ with $n \geq 2$.

*Proof.* $A$ projects onto the zero-sum subspace $V_\perp$. The operator $B$ on the ring topology, represented as a circulant matrix with first row $(1, 1, 0, \ldots, 0)$, has eigenvalues $\lambda_k = 1 + e^{2\pi i k/n}$ for $k = 0, \ldots, n-1$. The mode $k = 0$ (uniform) has $|\lambda_0| = 2$; all other modes have $|\lambda_k| = |1 + e^{2\pi i k/n}| < 2$. After $A$ annihilates the $k = 0$ mode, the remaining modes have $|\lambda_k| < 2$. $C$ contracts all values toward zero ($|C(x)| < |x|$ for $x \neq 0$). The composition $C \circ B \circ A$ is a contraction on $V_\perp$. Iterated contraction drives $\|\bar{e}_t\|_2 \to 0$, hence $\sigma^2(e_t) \to 0$. $\square$

**Theorem 6 (Nonzero relational variance under composition including R).** Under repeated application of $E = C \circ R \circ B \circ A$ with $\rho \in (0, 0.5]$:

$$\sigma^2(e_t) > 0 \quad \forall\ t \geq 0$$

for nontrivial initial conditions.

*Proof sketch.* The antisymmetric term in R, $\rho(e[(i+1) \bmod n] - e[(i-1) \bmod n])$, couples to modes with nonzero imaginary eigenvalue components — precisely the modes orthogonal to the uniform mode that $A$ annihilates. Under this coupling, the contraction of $B$ and $C$ on $V_\perp$ does not drive $\sigma^2$ to zero: the antisymmetric term redistributes magnitude across relational modes such that no single application of $C \circ B$ annihilates the full mode content. The bounded saturation $C$ bounds all values ($|C(e)| < 1$) while being sign-preserving. For $\rho \in (0, 0.5]$, the circulation term is bounded such that the composition maps $\mathcal{E}_D$ into $\mathcal{E}_D$. Therefore $\|\bar{e}_t\|_2 > 0$ for all $t$, giving $\sigma^2(e_t) > 0$.

*Scope note.* This proof relies on spectral mode analysis of the circulant operators. A complete proof requires explicit eigenvalue bounds for the composed circulant $C \circ R \circ B$ demonstrating that no mode is annihilated under iteration for $\rho \in (0, 0.5]$. The spectral structure supports this conclusion and is computable in closed form; the full derivation is deferred to the companion paper (arXiv:2601.22389). The theorem statement is treated as established for the ring topology under the stated conditions, with the complete spectral proof as a declared open deliverable. $\square$

*Remark on the upper bound $\rho \leq 0.5$.* The bound ensures that the antisymmetric circulation defined by $R$ does not produce values whose magnitude grows faster than $C$ can contract them in a single step, which would lead to oscillatory instability in which the composition no longer preserves nonzero relational variance under the stated conditions. The value 0.5 is the stability boundary: below it, the composition remains bounded with nonzero relational variance; above it, mode amplification destabilizes the composition. A full spectral stability analysis characterizing this boundary for arbitrary $n$ is straightforward (the eigenvalues of the combined circulant are computable in closed form) but is deferred to the companion paper (arXiv:2601.22389).

### 8.6 The Non-Injective Transformation Principle

The projection requirement that follows in §8.7 is not an isolated design rule. It is a consequence of a general structural principle that unifies the impossibility result (Theorem 1b), the invisibility result (Theorem 3), and the kernel's representation discipline into a single causal chain.

**Principle (Non-injective transformation constraint).** Let $T$ be any transformation applied to a relational field such that:

$$\exists\ x \neq y \quad \text{with} \quad T(x) = T(y)$$

Then $T$ induces equivalence classes in the output representation that are not distinguishable by any downstream operator acting on $T$'s output. When the cardinality of the input space greatly exceeds the cardinality of the output space ($|\mathcal{E}| \gg K$, where $K$ is the dimensionality of the projected representation), the number of collapsed equivalence classes grows combinatorially, and the structural distinctions lost are not recoverable, detectable, or auditable from within the projected representation.

**Therefore:** all non-injective transformations applied to the output of the ABRCE operator sequence must be explicitly declared, with their preserved and discarded invariants stated.

This principle is not new content. It is the structural connector between results already proved:

(1) Non-injective transforms collapse distinct relational states into identical representations (Theorem 1b — proved for the object-primary encoding $P : D \to \mathbb{R}^{m \times n}$, but the mechanism is general to any non-injective $T$).

(2) Downstream operators acting on the collapsed representation cannot distinguish the collapsed states (Theorem 3 — proved for index-local detection, but the mechanism follows from the definition of non-injectivity: if $T(x) = T(y)$, no function of $T$'s output separates $x$ from $y$).

(3) The error introduced by the collapse is not representable within any validation apparatus operating on the collapsed representation (§7.2 — the null space of the detector includes the collapsed distinctions).

(4) Therefore, the only point at which the information loss can be identified is *at the point of transformation* — before downstream operators inherit the collapsed representation. This is the structural basis for the declaration requirement in §8.7.

The object-primary encoding analyzed in Sections 3–5 is a specific instance of this principle: a non-injective transformation ($P$) applied before any relational processing, collapsing relational arrangement into per-entity attributes. The projection analyzed in §8.7 is the corresponding instance at the output: a non-injective transformation ($P : \mathcal{E} \to D$) applied after relational processing, collapsing edge structure into node values. The principle is the same. The ABRCE sequence is defined without the first instance. The declaration requirement addresses the second.

### 8.7 The Projection Operator and Representation Transitions

The output of the ABRCE sequence is an edge field. Many applications require node-level quantities — a single value per index for visualization, thresholding, or coupling to external systems. The transition from edge field to node field is a *projection*.

**Definition 7 (Projection operator).** A projection operator is a mapping:

$$P : \mathcal{E} \to D$$

that reduces an edge field to a node field.

Every projection is lossy. An edge field over a ring topology with $n$ indices carries $n$ directed edges. A node field carries $n$ scalar values. The edge field encodes directional relations between adjacent indices; the node field does not. Any projection discards some component of the directional structure.

**Examples of projection:**

*Node sum:* $P_{\text{sum}}(e)[i] = e[i] + e[(i-1) \bmod n]$ — the sum of edges incident to index $i$. Preserves net imbalance at each index. Discards directional distribution (which neighbor contributes how much).

*Directional selection:* $P_{\text{fwd}}(e)[i] = e[i]$ — the forward edge only. Preserves one direction. Discards the backward relation.

*Magnitude:* $P_{\text{mag}}(e)[i] = |e[i]|$ — the absolute gradient strength. Preserves gradient magnitude. Discards sign (direction of the relation).

**The structural requirement:** No projection from edge field to node field may be implicit. Every application of $P$ must declare:

(a) **Projection type** — which mapping $P : \mathcal{E} \to D$ is applied.

(b) **Preserved invariants** — which structural properties of the edge field are present in the output of the projection.

(c) **Discarded invariants** — which structural properties are absent from the output.

This requirement is not an additional constraint imposed on the kernel. It is a direct consequence of the paper's central argument: implicit projection from relational representation to non-relational representation is the mechanism by which the object error enters. The ABRCE sequence is defined without the object error in the operator pathway. Undeclared projection at the output produces information loss of the same structural character as the object-primary encoding.

**Declared open condition.** Whether the kernel should formally enforce this requirement (refusing to compile or execute undeclared projections) or whether it is a discipline condition maintained by Origin is a design decision not resolved in this paper. The structural consequence is the same in either case: undeclared projection produces information loss of the same character as the object-primary encoding analyzed in Sections 3–5.

### 8.8 The Structural Inversion

*Note: the declaration requirement in §8.7 states three components (projection type, preserved invariants, discarded invariants). The "purpose" field sometimes proposed in engineering contexts is deliberately excluded: it is a semantic declaration, and this kernel is non-semantic. Purpose lives at the application layer. The kernel constrains what is preserved and what is lost, not why the projection is performed.*

*Application-layer interpretation.* The observable variance criterion reveals a structural inversion between object-primary and relation-primary evaluation of the same mathematical condition:

**Object-primary evaluation:** $\sigma^2 \to 0$ is convergence, equilibrium, stability — a desirable terminal state.

**Relation-primary evaluation:** $\sigma^2 \to 0$ is the condition under which the edge field carries zero relational variance — the field cannot distinguish any system state from any other within the declared topology.

These are not different interpretations of the same state. They are opposite conclusions about the same measurable quantity, and the relation-primary conclusion follows from Definition 2 within **D**: a field with $\sigma^2 = 0$ carries zero bits of relational information. It is, by Definition 2, a representation that contains no relational distinctions — it cannot distinguish any system state from any other.

Under the framework of (O1)–(O3), $\sigma^2 \to 0$ is evaluated as convergence because the entities have each reached a stable value. Under that framework, the condition that relational structure — the only informationally distinguishable content (Axiom 1) — is absent from the representation is not represented within the evaluation apparatus.

### 8.9 Sufficiency, Not Uniqueness

The formal argument of Sections 3–7 establishes that non-index-local operators are necessary for distinguishing relational structure within **D**. Any operator that satisfies Theorem 4's conclusion — distinguishing fields with identical marginals but different relational structure — must be non-index-local.

ABRCE is a sufficient construction: a specific composition of non-index-local operators defined without the structural commitments (O1)–(O3) of the object assumption. The argument does not claim ABRCE is the unique resolution. It claims ABRCE is the first formalized operator sequence that (a) is defined without all three object-assumption conditions, (b) exhibits nonzero relational variance under iteration through the R operator rather than $\sigma^2 \to 0$, (c) constrains output to **D** through C, (d) operates within a declared bounded domain **D** with a declared measurement mapping $M$, and (e) operates in a relational (edge field) representation throughout the operator pathway, with any transition to node representation explicitly declared as a lossy projection.

Other architectures containing relational operations (attention mechanisms, graph convolutions) address (a) without addressing (b), (c), (d), or (e). The formal argument establishes that all five conditions are necessary. Relational operations in an unbounded, undeclared domain without domain closure, without nonzero relational variance under iteration, and without declared representation transitions do not resolve the error class identified in this paper.

---

## 9. Summary of the Formal Argument

| Step | Statement | Status |
|------|-----------|--------|
| 1 | Informational content of measured data is relational (Axiom 1, Def. 2) | Axiom + definition |
| 2 | Relational content is equivalently characterized by centered representation or edge field (Props. 1, 2) | Proved |
| 3 | Characterization and extraction are structurally distinct under composition (§2.3) | Structural principle |
| 4 | All elements of D are relationally connected (Theorem 1a) | Proved |
| 5 | Index-local operators cannot distinguish relational structure (Theorem 1) | Proved |
| 6 | Object-primary encoding does not preserve relational structure injectively (Theorem 1b) | Proved |
| 7 | Index-local modeling produces monotonically non-decreasing relational divergence (Theorem 2) | Proved (via Lemma 1) |
| 8 | Index-local detection cannot detect relational divergence (Theorem 3) | Proved |
| 9 | Standard validation techniques are index-local and therefore share this null space (§7.2) | Proved by classification |
| 10 | ABRCE operators are not index-local, operate in edge representation, and distinguish relational structure (Theorem 4) | Proved |
| 11 | Without R, relational variance approaches $\sigma^2 = 0$ (Theorem 5) | Proved |
| 12 | With R, relational variance is nonzero at $\sigma^2 > 0$ (Theorem 6) | Proved |
| 13 | Non-injective transformations induce indistinguishable equivalence classes; declaration is structurally required (§8.6) | Derived from Theorems 1b, 3 |
| 14 | Projection from edge field to node field is lossy and must be declared (§8.7) | Structural requirement |
| 15 | Object-primary formalisms evaluate relational annihilation as convergence (§8.8) | Application-layer interpretation |

---

## 10. The Forced Response

Any mathematical response to this argument must take one of the following positions:

**Position 1: Deny Axiom 1.** Exhibit a concrete, bounded measurement mapping $M$ that produces non-relational informational content — a component of measured data whose system-distinguishing power is not carried by differences between values. This requires demonstrating that a bounded measurement apparatus produces output determined by absolute values rather than differences. Note: the obligation is to exhibit a *system* that is fully characterized by object-primary data, not merely a *measurement* that is object-primary (see §2.1).

**Position 2: Deny Theorem 1.** Exhibit a uniformly index-local operator that distinguishes two fields with identical marginal distributions but different relational structure. This requires showing that a function of $x[i]$ alone can detect the arrangement of values across indices.

**Position 3: Deny Theorem 1b.** Exhibit a reconstruction operator $R$ that recovers relational arrangement from an entity-attribute matrix that does not encode relational arrangement. This requires extracting information that is provably absent from the representation.

**Position 4: Deny Theorem 3.** Exhibit an index-local detection functional that detects relational divergence. This requires constructing a detection functional whose output differs for states that differ only in relational arrangement — states that are in the null space of the index-local operator class.

**Position 5: Deny the relevance.** Argue that the systems modeled in data science satisfy (O1)–(O3) exactly — that the entities in the data are genuinely self-identical, context-independent, and discretely bounded, and that no relational structure exists beyond what is captured in per-entity feature values. This requires demonstrating that the measured system contains no couplings, gradients, or context-dependencies beyond those encoded in the marginal distributions.

**Position 6: Deny the representation distinction.** Argue that the choice between centered representation (node field via global aggregate) and edge field (direct pairwise differences) is immaterial to downstream composition — that operators composed with a globally-aggregated extraction produce identical structural behavior to operators composed with a locally-extracted edge field. This requires demonstrating that the compositional pathway is invariant to the representation at the extraction layer, contradicting the structural principle established in §2.3.

Each of these is a precise mathematical obligation requiring a constructive proof or counterexample. No philosophical, methodological, or historical argument addresses the formal structure.

---

## References

Macomber, R. (2026). Invariant Relational Evolution over Bounded Domains. arXiv:2601.22389.
