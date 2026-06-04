# Future Integration: ternary-automata

## Current State
Implements cellular automata with ternary states {0, 1, 2}: `TernaryRule` with Wolfram numbering (3^27 possible rules), rule simulation on 1D grids, pattern detection, cycle finding, entropy evolution tracking, and named rules (majority, identity, zero, mod-3 addition).

## Integration Opportunities

### With ternary-cell / construct-core
The ternary-cell grid IS a cellular automaton. Each cell updates based on its neighborhood during `tick()`. The `TernaryRule` lookup table becomes the cell's update rule. Different rules produce different behaviors: `majority()` creates consensus-seeking grids, custom rules create emergent patterns. `cycle_detection()` identifies when a room enters a stable oscillation — important for detecting livelock in construct fleets.

### With ternary-consensus
The `majority()` rule IS consensus. Run `TernaryRule::majority()` on a cell grid, and it converges to local majorities. This is a ternary majority vote algorithm — identical to `ternary-consensus`'s voting but operating spatially rather than temporally. The speed of convergence depends on the initial density of each state.

### With ternary-automata × ternary-graph
Run CA rules on arbitrary graph topologies rather than 1D grids. The neighborhood of each cell becomes its graph neighbors. This enables CA on room-as-codespace topologies where rooms aren't arranged in a line but in arbitrary graphs.

## Potential in Mature Systems
In PLATO, the cell grid runs a CA rule as its Layer 0 computation. The `TernaryRule` is stored as a 27-byte lookup table — fits in ESP32 ROM. Each tick reads the cell's three neighbors, indexes into the table, and outputs the new state. No floating point, no branches, no memory allocation. The `entropy_evolution()` function monitors whether the grid is becoming more ordered or chaotic — a diagnostic signal that propagates to Layer 1 for analysis.

## Cross-Pollination Ideas
**Music × Automata:** Wolfram's Rule 30 generates pseudo-random sequences. Apply this to ternary rhythm generation: each cell's state maps to a drum hit (0 = rest, 1 = tap, 2 = accent). Running Rule 30 on a circular grid produces evolving rhythmic patterns that never repeat exactly. This is algorithmic composition via CA — connects to `ternary-music`.

**Game of Life × Ternary:** Conway's Game of Life has a ternary analog: alive/dead/hibernating. The hibernating state adds persistence — a cell that "dies" enters hibernation and can be revived by neighbors. This models room occupancy (occupied/empty/reserved) with natural memory.

## Dependencies for Next Steps
- 2D CA grid support for spatial room layouts
- Graph-structured neighborhoods (not just linear)
- CA rule discovery: which rules produce useful room behaviors?
