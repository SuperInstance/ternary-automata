# ternary-automata

**Cellular automata with ternary states: Wolfram numbering, elementary rules, simulation, and pattern analysis for 3-state systems.**

`ternary-automata` extends classical two-state cellular automata (CA) to three states $\{0, 1, 2\}$, providing a complete framework for defining rules via Wolfram numbers, simulating evolution under various boundary conditions, detecting cycles, and measuring entropy evolution over time.

## Why It Matters

Cellular automata are the simplest known systems that produce emergent complexity from local rules. Stephen Wolfram's systematic study of binary CA (states $\{0, 1\}$, Rule 110) proved that even elementary rules can be **Turing complete**.

Extending to three states $\{0, 1, 2\}$ dramatically increases the rule space:

| Property | Binary CA | Ternary CA |
|----------|-----------|------------|
| Neighborhood size | $2^3 = 8$ | $3^3 = 27$ |
| Possible rules | $2^{2^3} = 256$ | $3^{3^3} = 3^{27} \approx 7.6 \times 10^{12}$ |
| Max Wolfram number | 255 | $3^{27} - 1 \approx 7.6 \times 10^{12}$ |

The vastly larger rule space enables richer behavior: more complex patterns, longer transients, and richer entropy dynamics — all from local neighborhood rules.

## How It Works

### Ternary Wolfram Numbering

Each ternary CA rule is a function mapping each of the 27 possible neighborhoods $(l, c, r) \in \{0,1,2\}^3$ to an output state $o \in \{0,1,2\}$. The neighborhood is encoded as a base-3 integer:

$$\text{idx}(l, c, r) = 9l + 3c + r$$

The **Wolfram number** $W$ is the base-3 representation of the output table:

$$W = \sum_{i=0}^{26} o_i \cdot 3^i$$

where $o_i$ is the output for neighborhood index $i$. The Wolfram number uniquely identifies each of the $\approx 7.6 \times 10^{12}$ possible ternary rules.

**Decoding:** Extract outputs via iterated modular arithmetic:

$$o_i = \left\lfloor \frac{W}{3^i} \right\rfloor \bmod 3$$

**Complexity:** $O(1)$ for table lookup (`apply_neighborhood`); $O(27)$ for rule construction from a Wolfram number.

### Named Rules

| Rule | Formula | Behavior |
|------|---------|----------|
| Identity | $o = c$ | Pass-through; no change |
| Zero | $o = 0$ | Annihilation; all cells become 0 |
| Majority | $o = \text{mode}(l, c, r)$ | Smoothing; converges to stable domains |
| Rule 110 (ternary) | $o = (l + c + r) \bmod 3$ | Additive; exhibits complex patterns |
| Left Shift | $o = l$ | Translation; shifts all cells right |
| XOR | $o = (l + c + r) \bmod 3$ | Same as ternary Rule 110 |

### Simulation and Boundary Conditions

The simulator advances the CA one generation at a time:

$$\text{state}^{(t+1)}_i = \text{rule}(\text{state}^{(t)}_{i-1},\; \text{state}^{(t)}_i,\; \text{state}^{(t)}_{i+1})$$

Three boundary conditions handle cells at the edges:

| Boundary | Left neighbor of cell 0 | Right neighbor of cell $n{-}1$ |
|----------|------------------------|-------------------------------|
| `Fixed(v)` | $v$ | $v$ |
| `Periodic` | $\text{state}_{n-1}$ (wrap) | $\text{state}_0$ (wrap) |
| `Zero` | $0$ | $0$ |

**Complexity:** $O(n)$ per generation step for a CA of width $n$; $O(n \cdot t)$ for $t$ steps of simulation.

### Cycle Detection

Since the state space is finite ($3^n$ possible states for width $n$), every CA must eventually enter a cycle. The simulator detects this by hashing each state and storing it in a map:

$$\text{state}^{(t)} \mapsto t$$

When a state repeats, the **preperiod** (transient length) and **period** are:

$$\text{preperiod} = t_{\text{first}}, \qquad \text{period} = t_{\text{current}} - t_{\text{first}}$$

**Complexity:** $O(T)$ time and $O(T)$ space, where $T$ is the preperiod + period, bounded by $\min(3^n, \text{max\_steps})$.

### Entropy Evolution

The **Shannon entropy** of the CA state measures its information content:

$$H(t) = -\sum_{s \in \{0,1,2\}} p_s(t) \cdot \log_2 p_s(t)$$

where $p_s(t) = n_s(t) / n$ is the fraction of cells in state $s$ at time $t$. The maximum entropy is $\log_2 3 \approx 1.585$ bits (uniform distribution over three states).

Tracking $H(t)$ over time reveals whether the CA converges to order ($H \to 0$), chaos ($H \to \log_2 3$), or complex intermediate states.

### Pattern Detection and Hamming Distance

- **Pattern detection:** Checks if the most recent state matches any prior state in the history.
- **Hamming distance:** $d_H(\mathbf{a}, \mathbf{b}) = |\{i : a_i \neq b_i\}|$ — counts positions where two states differ.

## Quick Start

```toml
[dependencies]
ternary-automata = "0.1"
```

```rust
use ternary_automata::{TernaryRule, CASimulator, Boundary, state_to_visual};

// Create the majority rule (smoothing/convergence)
let rule = TernaryRule::majority();

// Initial state: a perturbation in a uniform background
let initial: Vec<u8> = vec![0, 0, 0, 1, 2, 1, 0, 0, 0];

let mut sim = CASimulator::new(rule, initial, Boundary::Periodic);

// Step forward
sim.step_n(10);
println!("State: {}", state_to_visual(sim.state()));

// Detect cycles
if let Some((preperiod, period)) = sim.find_cycle(1000) {
    println!("Cycle: preperiod={}, period={}", preperiod, period);
}

// Track entropy evolution
let mut sim2 = CASimulator::new(TernaryRule::rule_110_ternary(), vec![0, 1, 2], Boundary::Periodic);
let entropies = sim2.entropy_evolution(50);
println!("Entropy: {:?}", entropies);
```

## API

| Type/Function | Purpose | Complexity |
|---------------|---------|------------|
| `TernaryRule` | CA rule with 27-entry lookup table | $O(1)$ lookup |
| `TernaryRule::from_wolfram()` | Decode rule from base-3 Wolfram number | $O(27)$ |
| `TernaryRule::identity/majority/rule_110_ternary/...` | Named rule constructors | $O(27)$ |
| `CASimulator` | CA simulation engine | $O(n)$ per step |
| `CASimulator::find_cycle()` | Cycle detection via hashing | $O(T)$ time/space |
| `CASimulator::entropy()` | Shannon entropy of current state | $O(n)$ |
| `CASimulator::entropy_evolution()` | Entropy time series | $O(n \cdot t)$ |
| `detect_pattern()` | Check for state repetition in history | $O(T)$ |
| `hamming_distance()` | Position-wise difference count | $O(n)$ |
| `state_to_visual()` | Unicode block rendering | $O(n)$ |

## Architecture Notes

Cellular automata are a direct physical instantiation of the SuperInstance conservation law **γ + η = C**. Each CA generation transforms the state vector, redistributing information between structure ($\gamma$ — ordered patterns) and randomness ($\eta$ — disordered states).

Rules like **Majority** minimize entropy: $\gamma \to C$, $\eta \to 0$, producing stable domains. Rules like **ternary Rule 110** operate at the edge of chaos: $\gamma \approx \eta$, producing complex structures at intermediate entropy. The entropy evolution curve $H(t)$ directly visualizes the $\gamma$/$\eta$ trade-off over time.

The finite state space guarantees eventual periodicity: the CA must settle into a cycle where $\gamma + \eta = C$ is exactly maintained — the system has found its equilibrium within the conservation law.

## References

- Wolfram, S. *A New Kind of Science.* Wolfram Media, 2002. — Elementary CA, Rule 110 universality, entropy classification.
- Cook, M. *Universality in Elementary Cellular Automata.* Complex Systems 15(1), 2004. — Proof that Rule 110 is Turing complete.
- von Neumann, J. *Theory of Self-Reproducing Automata.* (Ed. A.W. Burks.) Univ. of Illinois Press, 1966. — Foundational CA theory.
| Shannon, C.E. *A Mathematical Theory of Communication.* Bell System Technical Journal 27, 1948. — Entropy as information measure.
| Wolfram, S. *Statistical Mechanics of Cellular Automata.* Rev. Mod. Phys. 55(3), 1983. — Entropy evolution in CA.

## License

MIT
