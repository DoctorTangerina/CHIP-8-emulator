# CHIP-8: the 35 opcodes

Every opcode is 2 bytes long and stored **big-endian**.

## Notation

| Symbol | Meaning |
|---|---|
| `NNN` | 12-bit address |
| `NN` | 8-bit constant |
| `N` | 4-bit constant |
| `X`, `Y` | 4-bit register identifier |
| `VX` | One of the 16 data registers (`V0` to `VF`) |
| `I` | Address register |
| `PC` | Program counter |

> `VF` doubles as a flag: carry on addition, "no borrow" on subtraction, and collision on draw.

## Opcode table

| # | Opcode | Type | Pseudocode | Description |
|---|---|---|---|---|
| 1 | `0NNN` | Call | — | Calls an RCA 1802 machine-code routine at `NNN`. Not used by most ROMs (can be ignored). |
| 2 | `00E0` | Display | `disp_clear()` | Clears the screen. |
| 3 | `00EE` | Flow | `return;` | Returns from a subroutine (pops `PC` from the stack). |
| 4 | `1NNN` | Flow | `goto NNN;` | Jumps to address `NNN`. |
| 5 | `2NNN` | Flow | `*(0xNNN)()` | Calls the subroutine at `NNN` (pushes `PC` onto the stack). |
| 6 | `3XNN` | Cond | `if (VX == NN)` | Skips the next instruction if `VX` equals `NN`. |
| 7 | `4XNN` | Cond | `if (VX != NN)` | Skips the next instruction if `VX` does not equal `NN`. |
| 8 | `5XY0` | Cond | `if (VX == VY)` | Skips the next instruction if `VX` equals `VY`. |
| 9 | `6XNN` | Const | `VX = NN` | Sets `VX` to `NN`. |
| 10 | `7XNN` | Const | `VX += NN` | Adds `NN` to `VX` (does not affect `VF`). |
| 11 | `8XY0` | Assign | `VX = VY` | Copies `VY` into `VX`. |
| 12 | `8XY1` | BitOp | `VX \|= VY` | Bitwise OR. |
| 13 | `8XY2` | BitOp | `VX &= VY` | Bitwise AND. |
| 14 | `8XY3` | BitOp | `VX ^= VY` | Bitwise XOR. |
| 15 | `8XY4` | Math | `VX += VY` | Addition; `VF = 1` on carry, otherwise `0`. |
| 16 | `8XY5` | Math | `VX -= VY` | Subtracts `VY` from `VX`; `VF = 0` on borrow, otherwise `1`. |
| 17 | `8XY6` | BitOp | `VX >>= 1` | Shift right; `VF` gets the least significant bit before the shift. ⚠️ see quirks |
| 18 | `8XY7` | Math | `VX = VY - VX` | `VF = 0` on borrow, otherwise `1`. |
| 19 | `8XYE` | BitOp | `VX <<= 1` | Shift left; `VF` gets the most significant bit before the shift. ⚠️ see quirks |
| 20 | `9XY0` | Cond | `if (VX != VY)` | Skips the next instruction if `VX` does not equal `VY`. |
| 21 | `ANNN` | MEM | `I = NNN` | Sets `I` to `NNN`. |
| 22 | `BNNN` | Flow | `PC = V0 + NNN` | Jumps to `NNN + V0`. |
| 23 | `CXNN` | Rand | `VX = rand() & NN` | Random number (0 to 255) ANDed with `NN`. |
| 24 | `DXYN` | Display | `draw(VX, VY, N)` | Draws an 8-pixel-wide, `N`-pixel-tall sprite at (`VX`, `VY`), read from `I` (which is left unchanged). Pixels are XORed onto the screen; `VF = 1` if any lit pixel is turned off, otherwise `0`. |
| 25 | `EX9E` | KeyOp | `if (key() == VX)` | Skips the next instruction if the key in `VX` is pressed. |
| 26 | `EXA1` | KeyOp | `if (key() != VX)` | Skips the next instruction if the key in `VX` is **not** pressed. |
| 27 | `FX07` | Timer | `VX = get_delay()` | Reads the delay timer into `VX`. |
| 28 | `FX0A` | KeyOp | `VX = get_key()` | Blocks until a key is pressed, then stores it in `VX`. |
| 29 | `FX15` | Timer | `delay_timer(VX)` | Sets the delay timer to `VX`. |
| 30 | `FX18` | Sound | `sound_timer(VX)` | Sets the sound timer to `VX`. |
| 31 | `FX1E` | MEM | `I += VX` | Adds `VX` to `I`. ⚠️ see quirks |
| 32 | `FX29` | MEM | `I = sprite_addr[VX]` | Points `I` at the 4×5 font sprite for the hex digit (0 to F) in `VX`. |
| 33 | `FX33` | BCD | `set_BCD(VX)` | Stores the hundreds, tens, and ones digits of `VX` at `I`, `I+1`, and `I+2`. |
| 34 | `FX55` | MEM | `reg_dump(VX, &I)` | Stores `V0` through `VX` (inclusive) in memory starting at `I`. ⚠️ see quirks |
| 35 | `FX65` | MEM | `reg_load(VX, &I)` | Loads `V0` through `VX` (inclusive) from memory starting at `I`. ⚠️ see quirks |

## Quirks (differences between implementations)

The table follows the most common semantics today (based on the 1991 SUPER-CHIP spec). The original COSMAC VIP interpreter (1978) behaves differently in a few places:

- **`8XY6` / `8XYE`:** the original shifts `VY` and stores the result in `VX`. CHIP-48/SCHIP ignore `VY` and shift `VX` itself.
- **`FX55` / `FX65`:** the original (and CHIP-48) leave `I` incremented (`I = I + X + 1`). SCHIP leaves `I` unchanged.
- **`FX1E`:** almost every interpreter leaves `VF` alone. The exception is the Amiga interpreter, which sets `VF` on overflow (`I + VX > 0xFFF`). *Spacefight 2091!* depends on that behavior, while *Animal Race* depends on the opposite.
- **`8XY3`, `8XY6`, `8XY7`, `8XYE`:** undocumented in the original spec. They worked because `8XYN` opcodes were dispatched straight to the 1802's ALU.

> Tip: make these behaviors configurable (quirk flags). Timendus' *chip8-test-suite* includes a dedicated quirks test that shows which mode a ROM expects.

## Sources

- [CHIP-8 — Wikipedia (via Wikiwand)](https://www.wikiwand.com/en/CHIP-8)
- [CHIP-8 — Wikipedia mirror with the full table](https://bafybeiemxf5abjwjbikoz4mc3a3dla6ual3jsgpdr4cjr3oz3evfyavhwq.ipfs.4everland.io/wiki/CHIP-8.html)