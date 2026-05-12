// =============================================================
// operators.rs — Metatron Dynamics V4
// Invariant Relational Kernel: ABR
//
// Canonical Rust implementation.
//
// V4 Kernel:  E(x, ρ) = R(B(A(x)), ρ(A(x)))
// C is a declared projection, not a kernel operator.
//
// V3 scalar compatibility (ABRCE) preserved as special case.
//
// Domain: D := { x ∈ ℝⁿ | n < ∞ and |x[i]| < ∞ ∀ i }
// All quantifiers bounded over D. No claim beyond D.
//
// Operator non-agency: these operators do not act, cause,
// optimize, or enforce. They define invariant conditions
// of relational admissibility within D.
// =============================================================

// =============================================================
// 1. TYPES
// =============================================================

// --- Scalar 1D (V3 compatible) ---

/// One scalar value per cell on a ring.
#[derive(Clone, Debug)]
pub struct ScalarNodeField1D {
    pub data: Vec<f64>,
}

/// One directed edge per cell on a ring.
#[derive(Clone, Debug)]
pub struct ScalarEdgeField1D {
    pub data: Vec<f64>,
}

// --- Scalar 2D (V3 compatible) ---

/// One scalar value per cell on a 2D torus.
/// Row-major: data[i * cols + j]
#[derive(Clone, Debug)]
pub struct ScalarNodeField2D {
    pub data: Vec<f64>,
    pub rows: usize,
    pub cols: usize,
}

/// Four directed edges per cell on a 2D torus.
#[derive(Clone, Debug)]
pub struct ScalarEdgeField2D {
    pub north: Vec<f64>,
    pub south: Vec<f64>,
    pub east: Vec<f64>,
    pub west: Vec<f64>,
    pub rows: usize,
    pub cols: usize,
}

// --- Multi-Topology 1D (V4) ---

/// k-component vector field on a ring. n cells, k components.
/// Component c at cell i: data[c][i]
#[derive(Clone, Debug)]
pub struct VectorNodeField1D {
    pub data: Vec<Vec<f64>>,
    pub n: usize,
    pub k: usize,
}

/// Multi-topology edge field on a 1D ring.
/// spatial[c][i]: spatial edge for component c at cell i
/// comp[p][i]: component edge for pair p at cell i
#[derive(Clone, Debug)]
pub struct MultiTopoEdgeField1D {
    pub spatial: Vec<Vec<f64>>,
    pub comp: Vec<Vec<f64>>,
    pub n: usize,
    pub k: usize,
    pub comp_pairs: Vec<(usize, usize)>,
}

// --- Multi-Topology 2D (V4) ---

/// k-component vector field on a 2D torus.
/// data[c] is row-major for component c: data[c][i * cols + j]
#[derive(Clone, Debug)]
pub struct VectorNodeField2D {
    pub data: Vec<Vec<f64>>,
    pub rows: usize,
    pub cols: usize,
    pub k: usize,
}

/// Multi-topology edge field on a 2D torus.
/// Four spatial directions per component, plus component edges.
#[derive(Clone, Debug)]
pub struct MultiTopoEdgeField2D {
    pub north: Vec<Vec<f64>>,  // north[c][i*cols+j]
    pub south: Vec<Vec<f64>>,
    pub east: Vec<Vec<f64>>,
    pub west: Vec<Vec<f64>>,
    pub comp: Vec<Vec<f64>>,   // comp[p][i*cols+j]
    pub rows: usize,
    pub cols: usize,
    pub k: usize,
    pub comp_pairs: Vec<(usize, usize)>,
}

// =============================================================
// 2. COMPONENT TOPOLOGY CONSTRUCTORS
// =============================================================

/// All-pairs component topology for k components.
pub fn comp_topo_all_pairs(k: usize) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    for a in 0..k {
        for b in (a + 1)..k {
            pairs.push((a, b));
        }
    }
    pairs
}

/// Ring component topology: i adjacent to (i+1) mod k.
pub fn comp_topo_ring(k: usize) -> Vec<(usize, usize)> {
    (0..k).map(|i| (i, (i + 1) % k)).collect()
}

// =============================================================
// 3. INDEX HELPERS
// =============================================================

#[inline]
fn idx_1d(i: isize, n: usize) -> usize {
    ((i % n as isize + n as isize) % n as isize) as usize
}

#[inline]
fn idx_2d(i: isize, j: isize, rows: usize, cols: usize) -> usize {
    let ri = ((i % rows as isize + rows as isize) % rows as isize) as usize;
    let ci = ((j % cols as isize + cols as isize) % cols as isize) as usize;
    ri * cols + ci
}

// =============================================================
// 4. SCALAR 1D OPERATORS (V3 — ABRCE)
// =============================================================

pub mod scalar_1d {
    use super::*;

    pub fn operator_a(f: &ScalarNodeField1D) -> ScalarEdgeField1D {
        let n = f.data.len();
        let data = (0..n)
            .map(|i| f.data[i] - f.data[idx_1d(i as isize + 1, n)])
            .collect();
        ScalarEdgeField1D { data }
    }

    pub fn operator_b(g: &ScalarEdgeField1D) -> ScalarEdgeField1D {
        let n = g.data.len();
        let data = (0..n)
            .map(|i| g.data[i] + g.data[idx_1d(i as isize + 1, n)])
            .collect();
        ScalarEdgeField1D { data }
    }

    pub fn compute_rho(a: &ScalarEdgeField1D, rho_base: f64) -> Vec<f64> {
        a.data.iter()
            .map(|&e| {
                let m = e.abs();
                rho_base * m / (1.0 + m)
            })
            .collect()
    }

    pub fn operator_r(bg: &ScalarEdgeField1D, rho: &[f64]) -> ScalarEdgeField1D {
        let n = bg.data.len();
        let data = (0..n)
            .map(|i| {
                let fwd = bg.data[idx_1d(i as isize + 1, n)];
                let bwd = bg.data[idx_1d(i as isize - 1, n)];
                bg.data[i] + rho[i] * (fwd - bwd)
            })
            .collect();
        ScalarEdgeField1D { data }
    }

    pub fn operator_c(g: &ScalarEdgeField1D) -> ScalarEdgeField1D {
        let data = g.data.iter()
            .map(|&e| e / (1.0 + e.abs()))
            .collect();
        ScalarEdgeField1D { data }
    }

    /// V3 composition: E(x, ρ) = C(R(B(A(x)), ρ(A(x))))
    pub fn operator_e_v3(f: &ScalarNodeField1D, rho_base: f64) -> ScalarEdgeField1D {
        let a = operator_a(f);
        let rho = compute_rho(&a, rho_base);
        let b = operator_b(&a);
        let r = operator_r(&b, &rho);
        operator_c(&r)
    }

    /// V4 kernel: E(x, ρ) = R(B(A(x)), ρ(A(x)))  — no C
    pub fn operator_e(f: &ScalarNodeField1D, rho_base: f64) -> ScalarEdgeField1D {
        let a = operator_a(f);
        let rho = compute_rho(&a, rho_base);
        let b = operator_b(&a);
        operator_r(&b, &rho)
    }
}

// =============================================================
// 5. SCALAR 2D OPERATORS (V3 — ABRCE)
// =============================================================

pub mod scalar_2d {
    use super::*;

    pub fn operator_a(f: &ScalarNodeField2D) -> ScalarEdgeField2D {
        let (r, c) = (f.rows, f.cols);
        let len = r * c;
        let mut north = vec![0.0; len];
        let mut south = vec![0.0; len];
        let mut east = vec![0.0; len];
        let mut west = vec![0.0; len];

        for i in 0..r {
            for j in 0..c {
                let ij = idx_2d(i as isize, j as isize, r, c);
                let v = f.data[ij];
                north[ij] = v - f.data[idx_2d(i as isize - 1, j as isize, r, c)];
                south[ij] = v - f.data[idx_2d(i as isize + 1, j as isize, r, c)];
                east[ij] = v - f.data[idx_2d(i as isize, j as isize + 1, r, c)];
                west[ij] = v - f.data[idx_2d(i as isize, j as isize - 1, r, c)];
            }
        }
        ScalarEdgeField2D { north, south, east, west, rows: r, cols: c }
    }

    pub fn operator_b(g: &ScalarEdgeField2D) -> ScalarEdgeField2D {
        let (r, c) = (g.rows, g.cols);
        let len = r * c;
        let mut north = vec![0.0; len];
        let mut south = vec![0.0; len];
        let mut east = vec![0.0; len];
        let mut west = vec![0.0; len];

        for i in 0..r {
            for j in 0..c {
                let ij = idx_2d(i as isize, j as isize, r, c);
                north[ij] = g.north[ij] + g.north[idx_2d(i as isize - 1, j as isize, r, c)];
                south[ij] = g.south[ij] + g.south[idx_2d(i as isize + 1, j as isize, r, c)];
                east[ij] = g.east[ij] + g.east[idx_2d(i as isize, j as isize + 1, r, c)];
                west[ij] = g.west[ij] + g.west[idx_2d(i as isize, j as isize - 1, r, c)];
            }
        }
        ScalarEdgeField2D { north, south, east, west, rows: r, cols: c }
    }

    pub fn compute_rho(a: &ScalarEdgeField2D, rho_base: f64) -> Vec<f64> {
        let len = a.rows * a.cols;
        (0..len)
            .map(|i| {
                let m = a.north[i].abs()
                    .max(a.south[i].abs())
                    .max(a.east[i].abs())
                    .max(a.west[i].abs());
                rho_base * m / (1.0 + m)
            })
            .collect()
    }

    pub fn operator_r(bg: &ScalarEdgeField2D, rho: &[f64]) -> ScalarEdgeField2D {
        let (r, c) = (bg.rows, bg.cols);
        let len = r * c;
        let mut north = vec![0.0; len];
        let mut south = vec![0.0; len];
        let mut east = vec![0.0; len];
        let mut west = vec![0.0; len];

        for i in 0..r {
            for j in 0..c {
                let ij = idx_2d(i as isize, j as isize, r, c);
                let zonal_asym = bg.east[ij] - bg.west[ij];
                let merid_asym = bg.south[ij] - bg.north[ij];
                let rh = rho[ij];

                north[ij] = bg.north[ij] + rh * zonal_asym;
                south[ij] = bg.south[ij] + rh * zonal_asym;
                east[ij] = bg.east[ij] + rh * merid_asym;
                west[ij] = bg.west[ij] + rh * merid_asym;
            }
        }
        ScalarEdgeField2D { north, south, east, west, rows: r, cols: c }
    }

    pub fn operator_c(g: &ScalarEdgeField2D) -> ScalarEdgeField2D {
        let len = g.rows * g.cols;
        let mut north = vec![0.0; len];
        let mut south = vec![0.0; len];
        let mut east = vec![0.0; len];
        let mut west = vec![0.0; len];

        for i in 0..len {
            let m = g.north[i].abs()
                .max(g.south[i].abs())
                .max(g.east[i].abs())
                .max(g.west[i].abs());
            let d = 1.0 + m;
            north[i] = g.north[i] / d;
            south[i] = g.south[i] / d;
            east[i] = g.east[i] / d;
            west[i] = g.west[i] / d;
        }
        ScalarEdgeField2D { north, south, east, west, rows: g.rows, cols: g.cols }
    }

    /// V3: E(x, ρ) = C(R(B(A(x)), ρ(A(x))))
    pub fn operator_e_v3(f: &ScalarNodeField2D, rho_base: f64) -> ScalarEdgeField2D {
        let a = operator_a(f);
        let rho = compute_rho(&a, rho_base);
        let b = operator_b(&a);
        let r = operator_r(&b, &rho);
        operator_c(&r)
    }

    /// V4 kernel: E(x, ρ) = R(B(A(x)), ρ(A(x)))
    pub fn operator_e(f: &ScalarNodeField2D, rho_base: f64) -> ScalarEdgeField2D {
        let a = operator_a(f);
        let rho = compute_rho(&a, rho_base);
        let b = operator_b(&a);
        operator_r(&b, &rho)
    }
}

// =============================================================
// 6. MULTI-TOPOLOGY 1D OPERATORS (V4 — ABR)
// =============================================================

pub mod multi_1d {
    use super::*;

    pub fn operator_a(f: &VectorNodeField1D) -> MultiTopoEdgeField1D {
        let n = f.n;
        let k = f.k;

        // Spatial edges: per component
        let spatial: Vec<Vec<f64>> = (0..k)
            .map(|c| {
                (0..n)
                    .map(|i| f.data[c][i] - f.data[c][idx_1d(i as isize + 1, n)])
                    .collect()
            })
            .collect();

        // Component edges: per declared pair
        let comp: Vec<Vec<f64>> = f.comp_pairs()
            .iter()
            .map(|&(a, b)| {
                (0..n).map(|i| f.data[a][i] - f.data[b][i]).collect()
            })
            .collect();

        // Use default all-pairs if not externally specified
        let pairs = comp_topo_all_pairs(k);

        MultiTopoEdgeField1D { spatial, comp, n, k, comp_pairs: pairs }
    }

    pub fn operator_a_with_topo(
        f: &VectorNodeField1D,
        pairs: &[(usize, usize)],
    ) -> MultiTopoEdgeField1D {
        let n = f.n;
        let k = f.k;

        let spatial: Vec<Vec<f64>> = (0..k)
            .map(|c| {
                (0..n)
                    .map(|i| f.data[c][i] - f.data[c][idx_1d(i as isize + 1, n)])
                    .collect()
            })
            .collect();

        let comp: Vec<Vec<f64>> = pairs
            .iter()
            .map(|&(a, b)| {
                (0..n).map(|i| f.data[a][i] - f.data[b][i]).collect()
            })
            .collect();

        MultiTopoEdgeField1D {
            spatial, comp, n, k,
            comp_pairs: pairs.to_vec(),
        }
    }

    pub fn operator_b(g: &MultiTopoEdgeField1D) -> MultiTopoEdgeField1D {
        let n = g.n;

        let spatial: Vec<Vec<f64>> = g.spatial.iter()
            .map(|s| {
                (0..n)
                    .map(|i| s[i] + s[idx_1d(i as isize + 1, n)])
                    .collect()
            })
            .collect();

        let comp: Vec<Vec<f64>> = g.comp.iter()
            .map(|c| {
                (0..n)
                    .map(|i| c[i] + c[idx_1d(i as isize + 1, n)])
                    .collect()
            })
            .collect();

        MultiTopoEdgeField1D {
            spatial, comp, n: g.n, k: g.k,
            comp_pairs: g.comp_pairs.clone(),
        }
    }

    pub fn compute_rho(a: &MultiTopoEdgeField1D, rho_base: f64) -> Vec<f64> {
        let n = a.n;
        (0..n)
            .map(|i| {
                let mut m: f64 = 0.0;
                for s in &a.spatial {
                    m = m.max(s[i].abs());
                }
                for c in &a.comp {
                    m = m.max(c[i].abs());
                }
                rho_base * m / (1.0 + m)
            })
            .collect()
    }

    pub fn operator_r(bg: &MultiTopoEdgeField1D, rho: &[f64]) -> MultiTopoEdgeField1D {
        let n = bg.n;
        let k = bg.k;
        let pairs = &bg.comp_pairs;

        // Component edge asymmetry: forward - backward along ring
        let comp_asym: Vec<Vec<f64>> = bg.comp.iter()
            .map(|c| {
                (0..n)
                    .map(|i| {
                        c[idx_1d(i as isize + 1, n)] - c[idx_1d(i as isize - 1, n)]
                    })
                    .collect()
            })
            .collect();

        // --- Spatial edges receive component-edge asymmetry ---
        let mut spatial: Vec<Vec<f64>> = bg.spatial.clone();

        for (p_idx, &(a, b)) in pairs.iter().enumerate() {
            for i in 0..n {
                // Antisymmetric: +rho for first, -rho for second
                spatial[a][i] += rho[i] * comp_asym[p_idx][i];
                spatial[b][i] -= rho[i] * comp_asym[p_idx][i];
            }
        }

        // --- Component edges receive spatial-edge asymmetry ---
        let mut comp: Vec<Vec<f64>> = bg.comp.clone();

        for (p_idx, &(a, b)) in pairs.iter().enumerate() {
            for i in 0..n {
                let spatial_asym = bg.spatial[a][i] - bg.spatial[b][i];
                comp[p_idx][i] += rho[i] * spatial_asym;
            }
        }

        MultiTopoEdgeField1D {
            spatial, comp, n, k,
            comp_pairs: pairs.clone(),
        }
    }

    /// V4 kernel: E(x, ρ) = R(B(A(x)), ρ(A(x)))
    pub fn operator_e(
        f: &VectorNodeField1D,
        pairs: &[(usize, usize)],
        rho_base: f64,
    ) -> MultiTopoEdgeField1D {
        let a = operator_a_with_topo(f, pairs);
        let rho = compute_rho(&a, rho_base);
        let b = operator_b(&a);
        operator_r(&b, &rho)
    }
}

// =============================================================
// 7. MULTI-TOPOLOGY 2D OPERATORS (V4 — ABR)
// =============================================================

pub mod multi_2d {
    use super::*;

    pub fn operator_a(
        f: &VectorNodeField2D,
        pairs: &[(usize, usize)],
    ) -> MultiTopoEdgeField2D {
        let (r, c, k) = (f.rows, f.cols, f.k);
        let len = r * c;

        let mut north = vec![vec![0.0; len]; k];
        let mut south = vec![vec![0.0; len]; k];
        let mut east = vec![vec![0.0; len]; k];
        let mut west = vec![vec![0.0; len]; k];

        for comp in 0..k {
            for i in 0..r {
                for j in 0..c {
                    let ij = idx_2d(i as isize, j as isize, r, c);
                    let v = f.data[comp][ij];
                    north[comp][ij] = v - f.data[comp][idx_2d(i as isize - 1, j as isize, r, c)];
                    south[comp][ij] = v - f.data[comp][idx_2d(i as isize + 1, j as isize, r, c)];
                    east[comp][ij] = v - f.data[comp][idx_2d(i as isize, j as isize + 1, r, c)];
                    west[comp][ij] = v - f.data[comp][idx_2d(i as isize, j as isize - 1, r, c)];
                }
            }
        }

        let comp_edges: Vec<Vec<f64>> = pairs
            .iter()
            .map(|&(a, b)| {
                (0..len).map(|ij| f.data[a][ij] - f.data[b][ij]).collect()
            })
            .collect();

        MultiTopoEdgeField2D {
            north, south, east, west,
            comp: comp_edges,
            rows: r, cols: c, k,
            comp_pairs: pairs.to_vec(),
        }
    }

    pub fn operator_b(g: &MultiTopoEdgeField2D) -> MultiTopoEdgeField2D {
        let (r, c, k) = (g.rows, g.cols, g.k);
        let len = r * c;

        let mut north = vec![vec![0.0; len]; k];
        let mut south = vec![vec![0.0; len]; k];
        let mut east = vec![vec![0.0; len]; k];
        let mut west = vec![vec![0.0; len]; k];

        for comp in 0..k {
            for i in 0..r {
                for j in 0..c {
                    let ij = idx_2d(i as isize, j as isize, r, c);
                    north[comp][ij] = g.north[comp][ij]
                        + g.north[comp][idx_2d(i as isize - 1, j as isize, r, c)];
                    south[comp][ij] = g.south[comp][ij]
                        + g.south[comp][idx_2d(i as isize + 1, j as isize, r, c)];
                    east[comp][ij] = g.east[comp][ij]
                        + g.east[comp][idx_2d(i as isize, j as isize + 1, r, c)];
                    west[comp][ij] = g.west[comp][ij]
                        + g.west[comp][idx_2d(i as isize, j as isize - 1, r, c)];
                }
            }
        }

        // Component edges: accumulate along meridional direction
        // DECLARED OPEN CONDITION: accumulation direction is a
        // domain-dependent choice. Origin declares.
        let comp_edges: Vec<Vec<f64>> = g.comp.iter()
            .map(|ce| {
                let mut out = vec![0.0; len];
                for i in 0..r {
                    for j in 0..c {
                        let ij = idx_2d(i as isize, j as isize, r, c);
                        let next = idx_2d(i as isize + 1, j as isize, r, c);
                        out[ij] = ce[ij] + ce[next];
                    }
                }
                out
            })
            .collect();

        MultiTopoEdgeField2D {
            north, south, east, west,
            comp: comp_edges,
            rows: r, cols: c, k,
            comp_pairs: g.comp_pairs.clone(),
        }
    }

    pub fn compute_rho(a: &MultiTopoEdgeField2D, rho_base: f64) -> Vec<f64> {
        let len = a.rows * a.cols;
        (0..len)
            .map(|ij| {
                let mut m: f64 = 0.0;
                for comp in 0..a.k {
                    m = m.max(a.north[comp][ij].abs());
                    m = m.max(a.south[comp][ij].abs());
                    m = m.max(a.east[comp][ij].abs());
                    m = m.max(a.west[comp][ij].abs());
                }
                for ce in &a.comp {
                    m = m.max(ce[ij].abs());
                }
                // DECLARED OPEN CONDITION: single rho. Origin may
                // declare ρ_spatial and ρ_component independently.
                rho_base * m / (1.0 + m)
            })
            .collect()
    }

    pub fn operator_r(bg: &MultiTopoEdgeField2D, rho: &[f64]) -> MultiTopoEdgeField2D {
        let (r, c, k) = (bg.rows, bg.cols, bg.k);
        let len = r * c;
        let pairs = &bg.comp_pairs;

        // --- Spatial cross-axis coupling (within each component) ---
        let mut north = vec![vec![0.0; len]; k];
        let mut south = vec![vec![0.0; len]; k];
        let mut east = vec![vec![0.0; len]; k];
        let mut west = vec![vec![0.0; len]; k];

        for comp in 0..k {
            for ij in 0..len {
                let zonal_asym = bg.east[comp][ij] - bg.west[comp][ij];
                let merid_asym = bg.south[comp][ij] - bg.north[comp][ij];
                let rh = rho[ij];

                north[comp][ij] = bg.north[comp][ij] + rh * zonal_asym;
                south[comp][ij] = bg.south[comp][ij] + rh * zonal_asym;
                east[comp][ij] = bg.east[comp][ij] + rh * merid_asym;
                west[comp][ij] = bg.west[comp][ij] + rh * merid_asym;
            }
        }

        // --- Spatial edges receive component-edge asymmetry ---
        let cc_scale = 0.5;
        for (p_idx, &(a, b)) in pairs.iter().enumerate() {
            for i in 0..r {
                for j in 0..c {
                    let ij = idx_2d(i as isize, j as isize, r, c);
                    let rh = rho[ij];

                    // Component edge spatial asymmetry (meridional)
                    let comp_asym_ns =
                        bg.comp[p_idx][idx_2d(i as isize + 1, j as isize, r, c)]
                        - bg.comp[p_idx][idx_2d(i as isize - 1, j as isize, r, c)];

                    // Component edge spatial asymmetry (zonal)
                    let comp_asym_ew =
                        bg.comp[p_idx][idx_2d(i as isize, j as isize + 1, r, c)]
                        - bg.comp[p_idx][idx_2d(i as isize, j as isize - 1, r, c)];

                    // Meridional spatial edges receive zonal comp asymmetry
                    north[a][ij] += rh * cc_scale * comp_asym_ew;
                    north[b][ij] -= rh * cc_scale * comp_asym_ew;
                    south[a][ij] += rh * cc_scale * comp_asym_ew;
                    south[b][ij] -= rh * cc_scale * comp_asym_ew;

                    // Zonal spatial edges receive meridional comp asymmetry
                    east[a][ij] += rh * cc_scale * comp_asym_ns;
                    east[b][ij] -= rh * cc_scale * comp_asym_ns;
                    west[a][ij] += rh * cc_scale * comp_asym_ns;
                    west[b][ij] -= rh * cc_scale * comp_asym_ns;
                }
            }
        }

        // --- Component edges receive spatial-edge asymmetry ---
        let mut comp_out: Vec<Vec<f64>> = bg.comp.clone();
        for (p_idx, &(a, b)) in pairs.iter().enumerate() {
            for ij in 0..len {
                // DECLARED OPEN CONDITION: using north direction as
                // representative spatial edge for component coupling.
                // Origin may declare alternative or averaged form.
                let spatial_asym = bg.north[a][ij] - bg.north[b][ij];
                comp_out[p_idx][ij] += rho[ij] * spatial_asym;
            }
        }

        MultiTopoEdgeField2D {
            north, south, east, west,
            comp: comp_out,
            rows: r, cols: c, k,
            comp_pairs: pairs.clone(),
        }
    }

    /// V4 kernel: E(x, ρ) = R(B(A(x)), ρ(A(x)))
    pub fn operator_e(
        f: &VectorNodeField2D,
        pairs: &[(usize, usize)],
        rho_base: f64,
    ) -> MultiTopoEdgeField2D {
        let a = operator_a(f, pairs);
        let rho = compute_rho(&a, rho_base);
        let b = operator_b(&a);
        operator_r(&b, &rho)
    }
}

// =============================================================
// 8. DECLARED PROJECTIONS (V4)
//
// C is not a kernel operator for multi-component fields.
// Every projection is lossy. Each declares what it preserves
// and what it discards. No implicit reduction exists.
// =============================================================

pub mod projections {
    use super::*;

    #[inline]
    fn bound(x: f64) -> f64 {
        x / (1.0 + x.abs())
    }

    // --- 1D Multi-Topology Projections ---

    /// Per-edge-type bounding.
    ///
    /// Preserves: internal ratios within each edge type, sign,
    ///            independence between edge types.
    /// Discards:  magnitude relationships between edge types.
    pub fn c_per_type_1d(g: &MultiTopoEdgeField1D) -> MultiTopoEdgeField1D {
        let spatial = g.spatial.iter()
            .map(|s| s.iter().map(|&v| bound(v)).collect())
            .collect();
        let comp = g.comp.iter()
            .map(|c| c.iter().map(|&v| bound(v)).collect())
            .collect();
        MultiTopoEdgeField1D {
            spatial, comp,
            n: g.n, k: g.k,
            comp_pairs: g.comp_pairs.clone(),
        }
    }

    /// Shared-denominator bounding.
    ///
    /// Preserves: cross-type magnitude ratios, sign.
    /// Discards:  quiet edge types suppressed when one dominates.
    pub fn c_shared_1d(g: &MultiTopoEdgeField1D) -> MultiTopoEdgeField1D {
        let n = g.n;
        let denom: Vec<f64> = (0..n)
            .map(|i| {
                let mut m: f64 = 0.0;
                for s in &g.spatial { m = m.max(s[i].abs()); }
                for c in &g.comp { m = m.max(c[i].abs()); }
                1.0 + m
            })
            .collect();

        let spatial = g.spatial.iter()
            .map(|s| s.iter().enumerate().map(|(i, &v)| v / denom[i]).collect())
            .collect();
        let comp = g.comp.iter()
            .map(|c| c.iter().enumerate().map(|(i, &v)| v / denom[i]).collect())
            .collect();

        MultiTopoEdgeField1D {
            spatial, comp,
            n, k: g.k,
            comp_pairs: g.comp_pairs.clone(),
        }
    }

    /// Spatial-only bounding: bound spatial per-type,
    /// scale component edges to O(1) preserving ratios.
    ///
    /// Preserves: spatial coherence per component (bounded),
    ///            component edge magnitude ratios (scaled).
    /// Discards:  raw spatial magnitudes, component edges not
    ///            independently bounded.
    pub fn c_spatial_only_1d(g: &MultiTopoEdgeField1D) -> MultiTopoEdgeField1D {
        let n = g.n;

        let spatial = g.spatial.iter()
            .map(|s| s.iter().map(|&v| bound(v)).collect())
            .collect();

        // Scale component edges by global max to O(1)
        let mut comp_max: f64 = 0.0;
        for c in &g.comp {
            for &v in c { comp_max = comp_max.max(v.abs()); }
        }

        let comp = if comp_max > 0.0 {
            g.comp.iter()
                .map(|c| c.iter().map(|&v| v / comp_max).collect())
                .collect()
        } else {
            g.comp.clone()
        };

        MultiTopoEdgeField1D {
            spatial, comp,
            n, k: g.k,
            comp_pairs: g.comp_pairs.clone(),
        }
    }

    // --- 2D Multi-Topology Projections ---

    /// Per-edge-type bounding on 2D multi-topology field.
    pub fn c_per_type_2d(g: &MultiTopoEdgeField2D) -> MultiTopoEdgeField2D {
        let bv = |v: &[Vec<f64>]| -> Vec<Vec<f64>> {
            v.iter().map(|s| s.iter().map(|&x| bound(x)).collect()).collect()
        };
        MultiTopoEdgeField2D {
            north: bv(&g.north),
            south: bv(&g.south),
            east: bv(&g.east),
            west: bv(&g.west),
            comp: bv(&g.comp),
            rows: g.rows, cols: g.cols, k: g.k,
            comp_pairs: g.comp_pairs.clone(),
        }
    }

    /// Shared-denominator bounding on 2D multi-topology field.
    pub fn c_shared_2d(g: &MultiTopoEdgeField2D) -> MultiTopoEdgeField2D {
        let len = g.rows * g.cols;
        let denom: Vec<f64> = (0..len)
            .map(|ij| {
                let mut m: f64 = 0.0;
                for comp in 0..g.k {
                    m = m.max(g.north[comp][ij].abs());
                    m = m.max(g.south[comp][ij].abs());
                    m = m.max(g.east[comp][ij].abs());
                    m = m.max(g.west[comp][ij].abs());
                }
                for ce in &g.comp {
                    m = m.max(ce[ij].abs());
                }
                1.0 + m
            })
            .collect();

        let div = |v: &[Vec<f64>]| -> Vec<Vec<f64>> {
            v.iter()
                .map(|s| s.iter().enumerate().map(|(ij, &x)| x / denom[ij]).collect())
                .collect()
        };

        MultiTopoEdgeField2D {
            north: div(&g.north),
            south: div(&g.south),
            east: div(&g.east),
            west: div(&g.west),
            comp: div(&g.comp),
            rows: g.rows, cols: g.cols, k: g.k,
            comp_pairs: g.comp_pairs.clone(),
        }
    }
}

// =============================================================
// 9. DIAGNOSTIC: σ² (VARIANCE)
// =============================================================

pub mod diagnostics {
    use super::*;

    fn variance(data: &[f64]) -> f64 {
        let n = data.len() as f64;
        if n == 0.0 { return 0.0; }
        let mean = data.iter().sum::<f64>() / n;
        data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / n
    }

    // --- Scalar 1D ---

    pub fn sigma_sq_1d(g: &ScalarEdgeField1D) -> f64 {
        variance(&g.data)
    }

    // --- Scalar 2D ---

    pub fn sigma_sq_2d(g: &ScalarEdgeField2D) -> f64 {
        variance(&g.north) + variance(&g.south)
            + variance(&g.east) + variance(&g.west)
    }

    // --- Multi-Topology 1D ---

    pub fn sigma_sq_multi_1d_spatial(g: &MultiTopoEdgeField1D, comp: usize) -> f64 {
        variance(&g.spatial[comp])
    }

    pub fn sigma_sq_multi_1d_comp(g: &MultiTopoEdgeField1D, pair: usize) -> f64 {
        variance(&g.comp[pair])
    }

    pub fn sigma_sq_multi_1d_total(g: &MultiTopoEdgeField1D) -> f64 {
        let sv: f64 = g.spatial.iter().map(|s| variance(s)).sum();
        let cv: f64 = g.comp.iter().map(|c| variance(c)).sum();
        sv + cv
    }

    // --- Multi-Topology 2D ---

    pub fn sigma_sq_multi_2d_spatial(g: &MultiTopoEdgeField2D, comp: usize) -> f64 {
        variance(&g.north[comp]) + variance(&g.south[comp])
            + variance(&g.east[comp]) + variance(&g.west[comp])
    }

    pub fn sigma_sq_multi_2d_comp(g: &MultiTopoEdgeField2D, pair: usize) -> f64 {
        variance(&g.comp[pair])
    }

    pub fn sigma_sq_multi_2d_total(g: &MultiTopoEdgeField2D) -> f64 {
        let sv: f64 = (0..g.k)
            .map(|c| sigma_sq_multi_2d_spatial(g, c))
            .sum();
        let cv: f64 = g.comp.iter().map(|c| variance(c)).sum();
        sv + cv
    }
}

// =============================================================
// 10. VectorNodeField1D HELPER
// =============================================================

impl VectorNodeField1D {
    pub fn new(data: Vec<Vec<f64>>) -> Self {
        let k = data.len();
        let n = if k > 0 { data[0].len() } else { 0 };
        assert!(data.iter().all(|c| c.len() == n));
        assert!(data.iter().all(|c| c.iter().all(|v| v.is_finite())));
        VectorNodeField1D { data, n, k }
    }

    fn comp_pairs(&self) -> Vec<(usize, usize)> {
        comp_topo_all_pairs(self.k)
    }
}

impl VectorNodeField2D {
    pub fn new(data: Vec<Vec<f64>>, rows: usize, cols: usize) -> Self {
        let k = data.len();
        let len = rows * cols;
        assert!(data.iter().all(|c| c.len() == len));
        assert!(data.iter().all(|c| c.iter().all(|v| v.is_finite())));
        VectorNodeField2D { data, rows, cols, k }
    }
}

// =============================================================
// 11. PRE-OPERATOR TRANSFORMATION CONSTRAINT
//
// No transformation T : D → D may be applied prior to A if T
// alters pairwise differences between elements of D, unless T
// is declared as part of measurement mapping M.
//
// Admissible prior to A:
//   T(x) = x + c     (uniform shift)
//   T(x) = x / s     (uniform scaling, declared unit choice)
//
// All other transformations inadmissible prior to A unless
// declared within M with stated preserved/discarded invariants.
// =============================================================

// =============================================================
// 12. DECLARED OPEN CONDITIONS
//
// 1. ρ splitting: single ρ vs ρ_spatial + ρ_component
// 2. Component topology scaling for k > 3
// 3. B component-edge accumulation direction
// 4. Spatial dimensionality extension (3-torus)
// 5. Coordinate frame for component coupling (SM/MLT vs geo)
// 6. 2D R: representative spatial edge for component coupling
//    (currently north only; may average or declare per-domain)
// =============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn sin_ring(n: usize, freq: f64, amp: f64) -> Vec<f64> {
        (0..n).map(|i| amp * (2.0 * PI * freq * i as f64 / n as f64).sin()).collect()
    }

    fn cos_ring(n: usize, freq: f64, amp: f64) -> Vec<f64> {
        (0..n).map(|i| amp * (2.0 * PI * freq * i as f64 / n as f64).cos()).collect()
    }

    #[test]
    fn scalar_1d_bounded() {
        let f = ScalarNodeField1D { data: sin_ring(16, 1.0, 100.0) };
        let e = scalar_1d::operator_e_v3(&f, 0.3);
        assert!(e.data.iter().all(|&v| v.abs() < 1.0));
    }

    #[test]
    fn scalar_1d_kernel_unbounded() {
        let f = ScalarNodeField1D { data: sin_ring(16, 1.0, 100.0) };
        let e = scalar_1d::operator_e(&f, 0.3);
        // V4 kernel output is NOT bounded to (-1, 1)
        let max = e.data.iter().map(|v| v.abs()).fold(0.0_f64, f64::max);
        assert!(max > 1.0);
    }

    #[test]
    fn scalar_1d_r_sustains_variance() {
        let f = ScalarNodeField1D { data: sin_ring(16, 1.0, 10.0) };
        let a = scalar_1d::operator_a(&f);
        let b = scalar_1d::operator_b(&a);
        let rho = scalar_1d::compute_rho(&a, 0.3);
        let r = scalar_1d::operator_r(&b, &rho);
        let var_b = diagnostics::sigma_sq_1d(&b);
        let var_r = diagnostics::sigma_sq_1d(&r);
        assert!(var_r >= var_b, "R must sustain or increase variance");
    }

    #[test]
    fn multi_1d_identical_components() {
        let vals = sin_ring(16, 1.0, 10.0);
        let f = VectorNodeField1D::new(vec![vals.clone(), vals.clone()]);
        let pairs = comp_topo_all_pairs(2);
        let e = multi_1d::operator_e(&f, &pairs, 0.3);

        // Component edges should be ~zero for identical components
        let comp_max: f64 = e.comp[0].iter().map(|v| v.abs()).fold(0.0, f64::max);
        assert!(comp_max < 1e-10, "Identical components: comp edges must be ~0");

        // Spatial edges should be equal for both components
        for i in 0..e.n {
            assert!((e.spatial[0][i] - e.spatial[1][i]).abs() < 1e-10);
        }
    }

    #[test]
    fn multi_1d_r_sustains_coupling() {
        let p = sin_ring(16, 1.0, 50.0);
        let q = cos_ring(16, 2.0, 5.0);
        let f = VectorNodeField1D::new(vec![p, q]);
        let pairs = comp_topo_all_pairs(2);

        // With R
        let e_r = multi_1d::operator_e(&f, &pairs, 0.3);
        let var_r = diagnostics::sigma_sq_multi_1d_total(&e_r);

        // Without R (just A → B)
        let a = multi_1d::operator_a_with_topo(&f, &pairs);
        let b = multi_1d::operator_b(&a);
        let var_b = diagnostics::sigma_sq_multi_1d_total(&b);

        assert!(var_r > var_b, "R must sustain relational variance");
    }

    #[test]
    fn multi_1d_per_type_preserves_quiet() {
        let p = sin_ring(16, 1.0, 100.0);
        let q = cos_ring(16, 2.0, 1.0);
        let f = VectorNodeField1D::new(vec![p.clone(), q.clone()]);
        let pairs = comp_topo_all_pairs(2);
        let e = multi_1d::operator_e(&f, &pairs, 0.3);

        let pt = projections::c_per_type_1d(&e);
        let sh = projections::c_shared_1d(&e);

        let q_var_pt = diagnostics::sigma_sq_multi_1d_spatial(&pt, 1);
        let q_var_sh = diagnostics::sigma_sq_multi_1d_spatial(&sh, 1);

        // Per-type must preserve quiet component better than shared
        assert!(q_var_pt > q_var_sh * 10.0,
            "Per-type must preserve quiet component: pt={}, sh={}", q_var_pt, q_var_sh);
    }

    #[test]
    fn projections_bounded() {
        let p = sin_ring(16, 1.0, 1000.0);
        let q = cos_ring(16, 2.0, 1.0);
        let f = VectorNodeField1D::new(vec![p, q]);
        let pairs = comp_topo_all_pairs(2);
        let e = multi_1d::operator_e(&f, &pairs, 0.3);

        let pt = projections::c_per_type_1d(&e);
        for s in &pt.spatial {
            assert!(s.iter().all(|&v| v.abs() < 1.0));
        }
        for c in &pt.comp {
            assert!(c.iter().all(|&v| v.abs() < 1.0));
        }

        let sh = projections::c_shared_1d(&e);
        for s in &sh.spatial {
            assert!(s.iter().all(|&v| v.abs() < 1.0));
        }
        for c in &sh.comp {
            assert!(c.iter().all(|&v| v.abs() < 1.0));
        }
    }
}
