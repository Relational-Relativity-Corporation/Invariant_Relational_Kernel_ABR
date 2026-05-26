## 12. Ring Inadmissibility in Application (V4 Extension)

The preceding sections establish that operator admissibility requires declared topology. This section establishes an additional constraint: **the proof topology is not the application topology.**

The ring topology is the minimal connected periodic structure on which the Object Error theorems are established. It satisfies the admissibility conditions of §3.1 by construction. The theorems suggest the ring establishes a lower bound on relational sensitivity under the proof conditions (Object Error §8.2, Remark on topology choice). Additional couplings extend the adjacency set over which the operator composition is defined. However, richer topology can also introduce cancellation, spectral interference, or projection effects — the lower-bound relationship is established under the proof conditions and may require separate verification for specific graph classes.

Under V4, ring topology is retained as a proof reference. It is not carried forward as a default for applications. **There is no topological default in V4.** Every application declares its own spatial and component topology through Origin as part of M. Undeclared topology is inadmissible.

### 12.1 Periodic Boundary Violation

The ring wraps the last index to the first, creating an adjacency between the terminal node and the initial node. In systems with open boundaries — supply chains, causal cascades, hierarchical organizations, unidirectional flows — this adjacency does not exist in the observable system.

An operator acting on a ring computes edge values between the terminal and initial nodes. These edge values enter B's accumulation and R's cross-coupling. The relational field produced by the operators contains structure that M never declared from observables.

This is an admissibility violation at the level of the C → O → D chain (Triad §3.9.6): the declared domain contains relational structure that the measurement mapping never produced.

### 12.2 Directional Violation

The ring produces bidirectional symmetric edges. A(x)[i] = x[i] - x[(i+1) mod n] generates one directed edge per adjacency, but the ring's symmetry means the edge from i to i+1 and the implied traversal from i+1 to i are treated equivalently by B and R.

Real systems exhibit directionally distinct propagation modes. In a supply chain, downstream propagation (material flow: supplier → retailer) and upstream propagation (demand/information flow: retailer → supplier) are structurally different. They carry different observables, operate at different speeds, and couple differently to other components.

The ring collapses this distinction. Downstream and upstream propagation become the same operation on a symmetric topology. The operators cannot distinguish the direction of relational change because the topology does not encode directionality.

The operators are correct — they compute exactly what the declared topology specifies. The topology is what fails to represent the observable system.

### 12.3 Uniform Degree Violation

The ring imposes degree 2 at every node. Each index has exactly one forward neighbor and one backward neighbor.

Real systems have irregular degree. A port node receives from multiple suppliers (fan-in). A warehouse distributes to multiple downstream nodes (fan-out). Hub nodes have high degree. Terminal nodes have degree 1. The degree distribution of the operational graph encodes structural information about the system — bottlenecks, convergence points, distribution topology.

The ring erases this information. All nodes are topologically equivalent. The operators cannot distinguish a bottleneck from a terminal because the topology declares them as identical.

### 12.4 Computational Validity vs Representational Admissibility

The ring does not fail computationally. The operators execute correctly on a ring. A extracts edges. B accumulates. R cross-couples. The output is a valid edge field in D. Tests pass. Gamma is computable.

This is precisely why the ring risks representational inadmissibility in application when undeclared. It produces plausible, valid, structurally meaningless output. The violation is not computational — it is representational. The output represents a system that does not exist: one with periodic boundaries, symmetric propagation, and uniform degree.

### 12.5 Relational Evolution and Recurrence

Within D, relational evolution — the change in the edge field under iterated operator application — depends on the topology over which it is computed. On a ring, relational evolution is symmetric: perturbations propagate equally in both directions. On a directed graph, relational evolution follows the declared flow structure.

More fundamentally: exact relational recurrence is not observed in evolving systems. Relational evolution introduces state-dependent asymmetry that prevents exact return to the same relational configuration, even in systems exhibiting cyclic or oscillatory behavior. The ring's periodic closure implicitly assumes that traversal around the full index set returns to an equivalent relational state. Under evolution, it does not.

This does not assert that cycles, recurrence, or oscillatory behavior do not exist. It asserts that exact relational-state recurrence — return to an identical edge field configuration — is not observed in systems where relational evolution changes the state space through which recurrence occurs. Cyclic interaction systems with heterogeneous coupling, directional flow, and non-uniform propagation exhibit recurrence-like behavior without exact relational closure. Cyclicity is a dynamical property. Ring topology is a geometric property. They are not equivalent.

---

## 13. V4 Topology Declaration Requirements

### 13.1 Spatial Topology

Every V4 application declares its spatial topology as a directed graph with explicit adjacency lists. The declaration specifies:

- Which nodes are adjacent, in which direction
- Continuation rules at every node (which neighbors are forward and backward)
- Boundary behavior at terminal nodes (nodes with in-degree or out-degree of zero)
- Branch-point behavior (fan-in, fan-out)

Adjacency is declared, not inferred. Undeclared adjacency is not computed.

### 13.2 Directional Edge Types

Each declared spatial adjacency produces two structurally distinct edge types:

- **Downstream** (source → target): propagation in the declared flow direction
- **Upstream** (target → source): propagation against the declared flow direction

These are first-class edges with independent identity. They are not negations of each other after B and R act — R's cross-coupling breaks the initial antisymmetry produced by A.

Downstream and upstream edges may have:

- Different coupling strengths in R
- Different accumulation behavior in B
- Different baseline Gamma profiles
- Different detection sensitivity

These properties are declared by Origin as part of the topology specification.

### 13.3 Component Topology

Component topology specifies which measurement components are operationally coupled at each node. It is declared by Origin as part of M.

**All-pairs is not the default.** Origin declares which component pairs are adjacent based on the operational structure of the observable system. Undeclared component pairs are not coupled by R. If R surfaces indirect coupling through declared pairs, that is a result, not an input.

### 13.4 Irregular Degree

Nodes may have any degree consistent with the declared adjacency. Hub nodes, terminal nodes, fan-in nodes, and fan-out nodes are all admissible. The operators handle irregular degree through explicit adjacency lists, not through uniform index arithmetic.

`np.roll` and equivalent periodic-index operations are inadmissible in V4 application operator code. They silently assume ring topology.

---

## 14. Empirical Demonstration: Supply Chain

The `abr-vs-knowledge-graph` repository demonstrates the V4 topology discipline on a supply chain system with the following declared structure:

- **8 nodes** on a directed acyclic graph with 7 adjacencies
- **5 components** (inventory_level, lead_time, unit_cost, quality_score, throughput)
- **6 declared component pairs** out of 10 possible
- **Edge field dimensionality: 118** (35 downstream + 35 upstream + 48 component)

Nodes have irregular degree:

| Node | Out-degree | In-degree |
|------|-----------|----------|
| Supplier_Shanghai | 1 | 0 |
| Supplier_Vietnam | 1 | 0 |
| Port_LongBeach | 1 | 2 |
| Warehouse_West | 2 | 1 |
| Warehouse_Central | 1 | 1 |
| Warehouse_East | 1 | 1 |
| Retailer_BigBox | 0 | 1 |
| Retailer_Online | 0 | 1 |

Under a Shanghai supplier delay perturbation, the graph-native ABR operators detected sustained relational field restructuring (via topology-specific Gamma deviation in upstream lead_time) 1 step before a competent declared-relationship graph system detected scalar threshold crossing at the perturbation source.

The more significant comparison is downstream propagation: the graph system traced the perturbation node-by-node from t=32 to t=73. The ABR operators detected field restructuring at t=31, before the perturbation had physically arrived at any downstream node.

This result is conditional on declared simulation conditions, topology, and detection parameters. It demonstrates a specific representational distinction between continuous-field-operator processing and declared-relationship processing — not universal superiority of either approach.

---

## 15. Additional Declared Open Conditions (V4)

The following open conditions are introduced by the V4 topology discipline, in addition to those declared in §3 and §4:

1. **Operator spectral characterization on irregular graphs.** The Fourier-basis spectral analysis (B-admissible spectral concentration, scale resonance) is established for ring/torus topologies. Graph Laplacian eigenbasis characterization is required for irregular graphs.

2. **B accumulation at branch points.** When a node has multiple downstream continuations, B currently averages the continuation edges. Whether averaging, summing, or weighted combination is appropriate is domain-dependent. Origin declares.

3. **R coupling strength asymmetry.** Whether downstream and upstream edges should receive different ρ values (ρ_downstream vs ρ_upstream) is a declared open condition. Current implementation uses a single per-node ρ.

4. **Component-edge spatial accumulation direction.** Whether component edges accumulate along the downstream direction, upstream direction, or both is domain-dependent. Current implementation accumulates along downstream neighbors. Origin declares.

5. **Lower-bound relationship on general graphs.** The Object Error theorems suggest the ring establishes a lower bound on relational sensitivity under proof conditions. Whether this lower-bound property holds for all admissible graph classes, or whether specific graph structures can reduce sensitivity below the ring floor through cancellation or spectral interference, is a declared open condition requiring separate analysis.

---

*All statements bounded over D. No claim beyond D. Sections 12–15 extend the topology admissibility framework to V4 graph-native applications. They do not modify the results of sections 1–11, which remain valid for the declared periodic topologies under which they were established.*
