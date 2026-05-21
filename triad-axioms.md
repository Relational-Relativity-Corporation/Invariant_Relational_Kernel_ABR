# Axioms of Operational Admissibility

## Preamble

The ABRCE operator framework and the Triad protocol are mathematical structures. They can be stated, proved, and implemented without reference to the conditions under which a human operator can correctly use them. This document identifies and formalizes those conditions.

The following two axioms are not derived from the framework. They are **preconditions for admissible engagement** with it. A human operator who does not satisfy both axioms will produce framework outputs that are formally well-formed but operationally ungrounded — the operators execute, but the domain over which they are declared is not the domain the operator believes they are working in.

---

## Definitions

**Definition 1 (Bounded cognitive frame).** Let $\mathcal{F}_h$ denote the bounded information-processing domain of a single human being $h$. This domain is finite-dimensional, finite-bandwidth, and temporally bounded:

$\mathcal{F}_h \subset \lbrace x \in \mathbb{R}^n \mid n < \infty,\; |x_i| < \infty \;\forall\, i \rbrace$

$\mathcal{F}_h$ contains all and only the information accessible to $h$ through perception, cognition, and memory at any given time. It does not contain the internal states of other agents, the outputs of instruments $h$ has not read, or any information outside $h$'s perceptual and cognitive range.

**Definition 2 (Agency).** An operation $\phi: \mathcal{F}_h \to \mathcal{F}_h$ constitutes an act of agency by $h$ if and only if:

1. The selection of $\phi$ from the available operation space originates within $\mathcal{F}_h$
2. The domain and codomain of $\phi$ are contained in $\mathcal{F}_h$
3. No external process determines $\phi$ without $h$'s recognition and assent

**Definition 3 (Mathematical primitive).** A quantity $p$ is *primitive* within a formal system $S$ if $p$ is not defined as a function of other quantities in $S$. All other quantities in $S$ are derived from $p$ and the operations of $S$.

---

## Axiom 1 — Sole Agency

**Within $\mathcal{F}_h$, the only source of agency is $h$.**

Formally: Let $\Phi(\mathcal{F}_h)$ denote the set of all operations applied within $\mathcal{F}_h$. For every $\phi \in \Phi(\mathcal{F}_h)$:

$$\text{source}(\phi) = h$$

No operation within $h$'s bounded frame is admissibly attributed to any entity other than $h$ — not to a tool, a model, a co-agent, or an external authority. Tools may extend the range of $\mathcal{F}_h$ (an instrument reveals data not previously accessible) but the act of incorporating that data into $\mathcal{F}_h$ and operating on it is $h$'s agency.

**Consequence.** If $h$ attributes the source of an operation within $\mathcal{F}_h$ to an external entity $e \notin \mathcal{F}_h$, then the domain declaration required by operator **A** is ungrounded. Specifically:

$$\text{source}(\phi) \neq h \implies A(\mathcal{F}_h) \text{ is not admissibly declared}$$

The Triad protocol requires Origin (the human) to declare the domain before Generator and Verifier act. If Origin has displaced agency onto a tool or external authority, no domain declaration has occurred — the system is operating without **A**, and by the Object Error theorem, divergence is monotonically non-decreasing.

---

## Axiom 2 — Relational Primitive

**Within $\mathcal{F}_h$, relation is the sole mathematical primitive.**

Formally: Let $\mathcal{R}$ denote the space of relations over elements accessible within $\mathcal{F}_h$, and let $\mathcal{O}$ denote the space of objects (index-local quantities). Then:

$$\forall\, o \in \mathcal{O},\; \exists\, f: \mathcal{R} \to \mathcal{O} \text{ such that } o = f(r_1, r_2, \ldots, r_k)$$

but the converse does not hold:

$$\exists\, r \in \mathcal{R} \text{ such that } \nexists\, g: \mathcal{O} \to \mathcal{R} \text{ with } r = g(o_1, o_2, \ldots, o_m)$$

Objects are projections of relational structure. Relations are not recoverable from objects. This is not a preference or interpretation — it is the content of the Object Error theorem (arXiv:2601.22389), which proves that index-local operators possess a structural null space $\mathcal{N}$ containing relational information, and that this null space produces monotonically non-decreasing divergence invisible to any index-local validation.

**As an axiom** (rather than citing the theorem as proof), this asserts: the human operator commits to treating relation as primitive *before processing begins*. This is an operational commitment, not a claim to be evaluated after the fact.

**Consequence.** If $h$ constructs objects first and then seeks relations between them, $h$ has applied a non-injective projection $\pi: \mathcal{R} \to \mathcal{O}$ before the ABRCE operators act. Information in $\ker(\pi)$ is destroyed. Operator **A** then declares a domain over an already-impoverished input space, and the framework's detection capacity is bounded above by:

$$\text{detection capacity} \leq \dim(\mathcal{R}) - \dim(\ker(\pi))$$

Every preprocessing step that converts relational structure into object structure prior to operator **A** reduces the upper bound on what the framework can detect.

---

## Mutual Dependence

The two axioms are not independent in practice.

**Axiom 2 requires Axiom 1.** Recognizing relation as primitive requires $h$ to see that the choice to treat objects as primary was *$h$'s choice* — an act of agency with structural consequences. Without Axiom 1, $h$ attributes the object-primary frame to training, convention, or "how things are," and the choice is invisible. It cannot be revised because it is not recognized as a choice.

**Axiom 1 requires Axiom 2.** Recognizing sole agency requires $h$ to see themselves *in relation to* their tools, models, and collaborators — not as an isolated object making decisions in a vacuum. Agency is a relational property: it describes $h$'s position within a bounded frame relative to the operations being performed. Without Axiom 2, $h$ models agency as a property of the object "$h$" and misidentifies it as will, preference, or control — none of which are structurally well-defined within $\mathcal{F}_h$.

**Together.** Axiom 1 grounds the Triad (Origin can declare the domain). Axiom 2 grounds the operators (the input space preserves relational structure). Neither alone is sufficient. Both together constitute the minimal admissibility conditions for a human operator engaging with the ABRCE framework.

---

## Relationship to Existing Framework Components

| Component | Requires Axiom 1 | Requires Axiom 2 |
|---|---|---|
| Operator A (domain declaration) | Domain is declared by $h$'s agency | Domain is over relational, not object, space |
| Operator B (local accumulation) | — | Accumulates relational gradients, not object deltas |
| Operator R (circulation) | — | Antisymmetric structure exists only in relational space |
| Operator C (coherence) | — | Coherence is a relational, not index-local, property |
| Triad protocol | Origin = $h$, undisplaced | Origin recognizes the frame is relational |
| Object Error theorem | Proves consequence of violating Axiom 2 | Is the formal content of Axiom 2's claim |
| Origin Training | Trains Axiom 1 recognition | Trains Axiom 2 recognition |
| Dignity Net | Detects displacement of agency (Axiom 1 drift) | Detects reversion to object-primary (Axiom 2 drift) |

---

## Note on Status

These axioms are not provable within the ABRCE framework. They are preconditions for its correct application by a human operator. The framework itself is a mathematical structure that operates on whatever input it receives. The axioms determine whether the input is admissible — whether the human has provided the framework with the relational structure it was designed to process, from a domain that was actually declared by the person responsible for declaring it.

A system that satisfies neither axiom can still run the operators and produce numerical output. The output will be formally valid and structurally meaningless — precisely the condition the Object Error theorem characterizes.
