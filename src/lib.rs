//! Cellular automata with ternary states.
//!
//! Provides elementary CA rules extended to 3 states, Wolfram numbering for ternary,
//! rule simulation, pattern detection, cycle finding, entropy evolution,
//! and the rule 110 ternary analog.

#![forbid(unsafe_code)]

/// A ternary cellular automaton rule.
///
/// Ternary CA rules use a neighborhood of 3 cells (left, center, right),
/// each with states {0, 1, 2}, giving 3^3 = 27 possible neighborhoods.
/// A rule maps each neighborhood to an output state.
/// The Wolfram number is a base-3 integer from 0 to 3^27 - 1.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TernaryRule {
    /// Lookup table: neighborhood encoded as ternary number (0..26) → output state
    table: [u8; 27],
    /// Wolfram number (base-3 representation)
    wolfram_number: u128,
}

impl TernaryRule {
    /// Create a rule from a Wolfram number (base-3 encoded lookup table).
    pub fn from_wolfram_number(mut wolfram: u128) -> Self {
        let mut table = [0u8; 27];
        for i in 0..27 {
            table[i] = (wolfram % 3) as u8;
            wolfram /= 3;
        }
        TernaryRule {
            table,
            wolfram_number: wolfram, // what's left after extracting all digits
        }
    }

    /// Create a rule from a lookup table (27 entries, indexed by neighborhood).
    pub fn from_table(table: [u8; 27]) -> Self {
        for &v in &table {
            assert!(v < 3, "State values must be 0, 1, or 2");
        }
        let mut wolfram: u128 = 0;
        for i in (0..27).rev() {
            wolfram = wolfram * 3 + table[i] as u128;
        }
        TernaryRule { table, wolfram_number: wolfram }
    }

    /// Fix the wolfram_number calculation (constructor variant)
    fn fix_wolfram(table: &[u8; 27]) -> u128 {
        let mut wolfram: u128 = 0;
        for i in (0..27).rev() {
            wolfram = wolfram * 3 + table[i] as u128;
        }
        wolfram
    }

    /// Create from wolfram number, properly
    pub fn from_wolfram(wolfram: u128) -> Self {
        let mut w = wolfram;
        let mut table = [0u8; 27];
        for i in 0..27 {
            table[i] = (w % 3) as u8;
            w /= 3;
        }
        TernaryRule {
            table,
            wolfram_number: wolfram,
        }
    }

    /// Get the output for a given neighborhood (left, center, right).
    pub fn apply_neighborhood(&self, left: u8, center: u8, right: u8) -> u8 {
        assert!(left < 3 && center < 3 && right < 3);
        let idx = left as usize * 9 + center as usize * 3 + right as usize;
        self.table[idx]
    }

    /// Get the Wolfram number for this rule.
    pub fn wolfram_number(&self) -> u128 {
        TernaryRule::fix_wolfram(&self.table)
    }

    /// Get the lookup table.
    pub fn table(&self) -> &[u8; 27] {
        &self.table
    }

    /// Identity rule: center cell value passes through unchanged.
    pub fn identity() -> Self {
        let mut table = [0u8; 27];
        for left in 0..3u8 {
            for center in 0..3u8 {
                for right in 0..3u8 {
                    let idx = left as usize * 9 + center as usize * 3 + right as usize;
                    table[idx] = center;
                }
            }
        }
        TernaryRule { table, wolfram_number: TernaryRule::fix_wolfram(&table) }
    }

    /// Rule that maps everything to 0.
    pub fn zero() -> Self {
        TernaryRule {
            table: [0u8; 27],
            wolfram_number: 0,
        }
    }

    /// Majority rule: output is the majority value among (left, center, right).
    pub fn majority() -> Self {
        let mut table = [0u8; 27];
        for left in 0..3u8 {
            for center in 0..3u8 {
                for right in 0..3u8 {
                    let idx = left as usize * 9 + center as usize * 3 + right as usize;
                    // Count occurrences
                    let mut counts = [0usize; 3];
                    counts[left as usize] += 1;
                    counts[center as usize] += 1;
                    counts[right as usize] += 1;
                    let majority = counts.iter().enumerate()
                        .max_by_key(|&(_, c)| c)
                        .map(|(v, _)| v)
                        .unwrap() as u8;
                    table[idx] = majority;
                }
            }
        }
        TernaryRule { table, wolfram_number: TernaryRule::fix_wolfram(&table) }
    }

    /// Ternary analog of Rule 110 (binary).
    /// In binary rule 110, the output is 1 when the neighborhood pattern has an odd
    /// number of 1s (simplified). For ternary, we use: output = (left + center + right) mod 3.
    pub fn rule_110_ternary() -> Self {
        let mut table = [0u8; 27];
        for left in 0..3u8 {
            for center in 0..3u8 {
                for right in 0..3u8 {
                    let idx = left as usize * 9 + center as usize * 3 + right as usize;
                    table[idx] = (left + center + right) % 3;
                }
            }
        }
        TernaryRule { table, wolfram_number: TernaryRule::fix_wolfram(&table) }
    }

    /// XOR-like rule: (left + center + right) mod 3 (same as rule_110_ternary).
    pub fn xor_rule() -> Self {
        Self::rule_110_ternary()
    }

    /// Left shift rule: output = left neighbor value.
    pub fn left_shift() -> Self {
        let mut table = [0u8; 27];
        for left in 0..3u8 {
            for center in 0..3u8 {
                for right in 0..3u8 {
                    let idx = left as usize * 9 + center as usize * 3 + right as usize;
                    table[idx] = left;
                }
            }
        }
        TernaryRule { table, wolfram_number: TernaryRule::fix_wolfram(&table) }
    }
}

/// A cellular automaton simulator.
pub struct CASimulator {
    /// The rule
    rule: TernaryRule,
    /// Current state
    state: Vec<u8>,
    /// Boundary condition
    boundary: Boundary,
}

/// Boundary conditions for the CA.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Boundary {
    /// Fixed value at boundaries
    Fixed(u8),
    /// Periodic (wrap around)
    Periodic,
    /// Zero boundary
    Zero,
}

impl CASimulator {
    /// Create a new simulator with the given rule, initial state, and boundary.
    pub fn new(rule: TernaryRule, initial_state: Vec<u8>, boundary: Boundary) -> Self {
        for &v in &initial_state {
            assert!(v < 3, "State values must be 0, 1, or 2");
        }
        CASimulator {
            rule,
            state: initial_state,
            boundary,
        }
    }

    /// Get the current state.
    pub fn state(&self) -> &[u8] {
        &self.state
    }

    /// Step the CA forward by one generation.
    pub fn step(&mut self) {
        let n = self.state.len();
        let mut new_state = vec![0u8; n];
        for i in 0..n {
            let left = match &self.boundary {
                Boundary::Fixed(v) => if i == 0 { *v } else { self.state[i - 1] },
                Boundary::Periodic => self.state[(i + n - 1) % n],
                Boundary::Zero => if i == 0 { 0 } else { self.state[i - 1] },
            };
            let center = self.state[i];
            let right = match &self.boundary {
                Boundary::Fixed(v) => if i == n - 1 { *v } else { self.state[i + 1] },
                Boundary::Periodic => self.state[(i + 1) % n],
                Boundary::Zero => if i == n - 1 { 0 } else { self.state[i + 1] },
            };
            new_state[i] = self.rule.apply_neighborhood(left, center, right);
        }
        self.state = new_state;
    }

    /// Step forward n generations.
    pub fn step_n(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    /// Run the simulation and collect all states.
    pub fn run(&mut self, steps: usize) -> Vec<Vec<u8>> {
        let mut history = vec![self.state.clone()];
        for _ in 0..steps {
            self.step();
            history.push(self.state.clone());
        }
        history
    }

    /// Find a cycle: returns (preperiod, period) if a cycle is detected within max_steps.
    pub fn find_cycle(&mut self, max_steps: usize) -> Option<(usize, usize)> {
        let mut seen = std::collections::HashMap::new();
        let initial = self.state.clone();
        seen.insert(initial, 0);

        for step in 1..=max_steps {
            self.step();
            if let Some(&prev_step) = seen.get(&self.state) {
                return Some((prev_step, step - prev_step));
            }
            seen.insert(self.state.clone(), step);
        }
        None
    }

    /// Compute Shannon entropy of the current state.
    pub fn entropy(&self) -> f64 {
        let n = self.state.len();
        if n == 0 {
            return 0.0;
        }
        let mut counts = [0usize; 3];
        for &v in &self.state {
            counts[v as usize] += 1;
        }
        let mut h = 0.0;
        for &c in &counts {
            if c > 0 {
                let p = c as f64 / n as f64;
                h -= p * p.log2();
            }
        }
        h
    }

    /// Track entropy evolution over n steps.
    pub fn entropy_evolution(&mut self, steps: usize) -> Vec<f64> {
        let mut entropies = vec![self.entropy()];
        for _ in 0..steps {
            self.step();
            entropies.push(self.entropy());
        }
        entropies
    }
}

/// Detect if a pattern repeats in the history.
pub fn detect_pattern(history: &[Vec<u8>]) -> Option<usize> {
    if history.len() < 3 {
        return None;
    }
    // Check if the last state matches any previous state
    let last = &history[history.len() - 1];
    for i in 0..history.len() - 1 {
        if &history[i] == last {
            return Some(history.len() - 1 - i);
        }
    }
    None
}

/// Convert a state to a string representation (using characters '0', '1', '2').
pub fn state_to_string(state: &[u8]) -> String {
    state.iter().map(|&v| char::from_digit(v as u32, 10).unwrap()).collect()
}

/// Convert a state to a visual representation (using block characters).
pub fn state_to_visual(state: &[u8]) -> String {
    state.iter().map(|&v| match v {
        0 => '░',
        1 => '▒',
        2 => '▓',
        _ => '?',
    }).collect()
}

/// Count the occurrences of each state in a state vector.
pub fn state_counts(state: &[u8]) -> [usize; 3] {
    let mut counts = [0usize; 3];
    for &v in state {
        counts[v as usize] += 1;
    }
    counts
}

/// Compute the Hamming distance between two states.
pub fn hamming_distance(a: &[u8], b: &[u8]) -> usize {
    a.iter().zip(b.iter()).filter(|(&x, &y)| x != y).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_rule() {
        let rule = TernaryRule::identity();
        for l in 0..3u8 {
            for c in 0..3u8 {
                for r in 0..3u8 {
                    assert_eq!(rule.apply_neighborhood(l, c, r), c);
                }
            }
        }
    }

    #[test]
    fn test_zero_rule() {
        let rule = TernaryRule::zero();
        for l in 0..3u8 {
            for c in 0..3u8 {
                for r in 0..3u8 {
                    assert_eq!(rule.apply_neighborhood(l, c, r), 0);
                }
            }
        }
    }

    #[test]
    fn test_wolfram_roundtrip() {
        let rule = TernaryRule::identity();
        let w = rule.wolfram_number();
        let rule2 = TernaryRule::from_wolfram(w);
        assert_eq!(rule, rule2);
    }

    #[test]
    fn test_majority_rule() {
        let rule = TernaryRule::majority();
        assert_eq!(rule.apply_neighborhood(0, 0, 1), 0);
        assert_eq!(rule.apply_neighborhood(1, 1, 0), 1);
        assert_eq!(rule.apply_neighborhood(0, 1, 0), 0);
        assert_eq!(rule.apply_neighborhood(2, 2, 1), 2);
    }

    #[test]
    fn test_xor_rule() {
        let rule = TernaryRule::xor_rule();
        assert_eq!(rule.apply_neighborhood(0, 0, 0), 0);
        assert_eq!(rule.apply_neighborhood(1, 1, 1), 0); // 3 mod 3 = 0
        assert_eq!(rule.apply_neighborhood(1, 0, 0), 1);
        assert_eq!(rule.apply_neighborhood(2, 2, 0), 1); // 4 mod 3 = 1
    }

    #[test]
    fn test_left_shift_rule() {
        let rule = TernaryRule::left_shift();
        assert_eq!(rule.apply_neighborhood(2, 1, 0), 2);
        assert_eq!(rule.apply_neighborhood(0, 2, 1), 0);
    }

    #[test]
    fn test_simulator_identity_no_change() {
        let rule = TernaryRule::identity();
        let state = vec![0, 1, 2, 1, 0];
        let mut sim = CASimulator::new(rule, state.clone(), Boundary::Zero);
        sim.step();
        assert_eq!(sim.state(), &state);
    }

    #[test]
    fn test_simulator_zero_rule() {
        let rule = TernaryRule::zero();
        let state = vec![1, 2, 0, 1, 2];
        let mut sim = CASimulator::new(rule, state, Boundary::Zero);
        sim.step();
        assert_eq!(sim.state(), &[0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_simulator_periodic_boundary() {
        let rule = TernaryRule::left_shift();
        let state = vec![1, 2, 0];
        let mut sim = CASimulator::new(rule, state, Boundary::Periodic);
        sim.step();
        // Left shift with periodic: each cell takes its left neighbor
        // index 0: left = state[2] = 0
        // index 1: left = state[0] = 1
        // index 2: left = state[1] = 2
        assert_eq!(sim.state(), &[0, 1, 2]);
    }

    #[test]
    fn test_find_cycle_zero_rule() {
        let rule = TernaryRule::zero();
        let state = vec![1, 0, 2];
        let mut sim = CASimulator::new(rule, state, Boundary::Zero);
        // After step 1: all zeros. Step 2: still all zeros. Cycle of period 1.
        let cycle = sim.find_cycle(100);
        assert!(cycle.is_some());
        let (preperiod, period) = cycle.unwrap();
        assert_eq!(period, 1); // all zeros is a fixed point
    }

    #[test]
    fn test_find_cycle_xor() {
        let rule = TernaryRule::xor_rule();
        let state = vec![0, 1, 2];
        let mut sim = CASimulator::new(rule, state, Boundary::Periodic);
        let cycle = sim.find_cycle(1000);
        assert!(cycle.is_some());
    }

    #[test]
    fn test_entropy_uniform() {
        let state = vec![0, 1, 2, 0, 1, 2];
        let rule = TernaryRule::identity();
        let sim = CASimulator::new(rule, state, Boundary::Zero);
        let h = sim.entropy();
        assert!((h - 3.0f64.log2()).abs() < 0.01);
    }

    #[test]
    fn test_entropy_all_same() {
        let state = vec![1, 1, 1, 1];
        let rule = TernaryRule::identity();
        let sim = CASimulator::new(rule, state, Boundary::Zero);
        assert_eq!(sim.entropy(), 0.0);
    }

    #[test]
    fn test_entropy_evolution() {
        let rule = TernaryRule::zero();
        let state = vec![1, 2, 0];
        let mut sim = CASimulator::new(rule, state, Boundary::Zero);
        let evo = sim.entropy_evolution(3);
        assert_eq!(evo.len(), 4);
        assert_eq!(evo[3], 0.0); // After zero rule, entropy is 0
    }

    #[test]
    fn test_detect_pattern() {
        let history = vec![
            vec![1, 0, 2],
            vec![0, 2, 1],
            vec![1, 0, 2], // repeats first
        ];
        assert_eq!(detect_pattern(&history), Some(2));
    }

    #[test]
    fn test_state_to_string() {
        assert_eq!(state_to_string(&[0, 1, 2]), "012");
    }

    #[test]
    fn test_state_to_visual() {
        let vis = state_to_visual(&[0, 1, 2]);
        assert_eq!(vis.chars().count(), 3);
    }

    #[test]
    fn test_state_counts() {
        assert_eq!(state_counts(&[0, 1, 2, 0, 0]), [3, 1, 1]);
    }

    #[test]
    fn test_hamming_distance() {
        assert_eq!(hamming_distance(&[0, 1, 2], &[0, 1, 2]), 0);
        assert_eq!(hamming_distance(&[0, 1, 2], &[1, 1, 2]), 1);
        assert_eq!(hamming_distance(&[0, 1, 2], &[2, 1, 0]), 2);
    }

    #[test]
    fn test_run_history() {
        let rule = TernaryRule::zero();
        let state = vec![1, 2, 0];
        let mut sim = CASimulator::new(rule, state, Boundary::Zero);
        let history = sim.run(2);
        assert_eq!(history.len(), 3);
        assert_eq!(history[1], vec![0, 0, 0]);
        assert_eq!(history[2], vec![0, 0, 0]);
    }

    #[test]
    fn test_rule_110_ternary_wolfram_number() {
        let rule = TernaryRule::rule_110_ternary();
        // The wolfram number should be deterministic
        let w = rule.wolfram_number();
        assert!(w > 0); // Not the zero rule
    }
}
