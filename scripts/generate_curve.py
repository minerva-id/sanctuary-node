#!/usr/bin/env python3
"""
Sanctuary Protocol v2.0 - Emission Curve Generator

Generates a pre-computed emission table based on the Sigmoid function:
S(t) = S_max / (1 + e^(-k(t - t_midpoint)))

The output is a Rust array that can be directly pasted into the runtime.
"""

import math

# ═══════════════════════════════════════════════════════════════════════════
# UNIVERSAL CONSTANTS (The DNA of Sanctuary)
# ═══════════════════════════════════════════════════════════════════════════
PI  = math.pi        # 3.141592653589793
E   = math.e         # 2.718281828459045
PHI = (1 + math.sqrt(5)) / 2  # 1.618033988749895

# Maximum Supply in smallest units (18 decimals like ETH)
# S_MAX = floor(π × e × φ × 10^6) × 10^18
S_MAX_UNITS = int(math.floor(PI * E * PHI * 1_000_000))  # 13,817,580
S_MAX = S_MAX_UNITS * (10**18)  # In planck (smallest unit)

# ═══════════════════════════════════════════════════════════════════════════
# EMISSION SCHEDULE PARAMETERS
# ═══════════════════════════════════════════════════════════════════════════
# Block time: 6 seconds
# Blocks per year: 365.25 * 24 * 60 * 60 / 6 = 5,256,000
BLOCKS_PER_YEAR = 5_256_000
BLOCK_TIME_SECONDS = 6

# Emission duration: 20 years for the curve to reach ~99% of S_MAX
TOTAL_YEARS = 20
TOTAL_BLOCKS = TOTAL_YEARS * BLOCKS_PER_YEAR  # 105,120,000 blocks

# Midpoint (inflection point) - when 50% of supply is emitted
MIDPOINT = TOTAL_BLOCKS / 2  # At 10 years

# Growth rate (k) - controls curve steepness
# Set so S(0) ≈ 0.01% and S(end) ≈ 99.99%
k = 10 / MIDPOINT

# Era duration (for reward table granularity)
# 1 Era = 24 hours = 14,400 blocks (at 6s block time)
BLOCKS_PER_ERA = 14_400
TOTAL_ERAS = int(TOTAL_BLOCKS / BLOCKS_PER_ERA)  # 7,300 eras (20 years)

print("╔══════════════════════════════════════════════════════════════════╗")
print("║    SANCTUARY PROTOCOL v2.0 - EMISSION TABLE GENERATOR           ║")
print("╠══════════════════════════════════════════════════════════════════╣")
print(f"║  Max Supply: {S_MAX_UNITS:,} SANC ({S_MAX_UNITS / 1e6:.2f}M)                        ║")
print(f"║  Emission Duration: {TOTAL_YEARS} years ({TOTAL_BLOCKS:,} blocks)         ║")
print(f"║  Eras: {TOTAL_ERAS:,} (1 era = 24 hours)                             ║")
print(f"║  Growth Rate (k): {k:.10f}                                 ║")
print("╚══════════════════════════════════════════════════════════════════╝")
print()

# ═══════════════════════════════════════════════════════════════════════════
# GENERATE EMISSION TABLE
# ═══════════════════════════════════════════════════════════════════════════
rewards_per_era = []
prev_supply = 0
peak_reward = 0
peak_era = 0

for era in range(1, TOTAL_ERAS + 1):
    t = era * BLOCKS_PER_ERA
    
    # Sigmoid function: S(t) = S_max / (1 + e^(-k(t - midpoint)))
    exponent = -k * (t - MIDPOINT)
    # Clamp to prevent overflow
    if exponent > 700:
        current_supply = 0
    elif exponent < -700:
        current_supply = S_MAX
    else:
        current_supply = S_MAX / (1 + math.exp(exponent))
    
    # New tokens to mint this era
    mint_amount = current_supply - prev_supply
    
    # Reward per block in this era
    reward_per_block = int(mint_amount / BLOCKS_PER_ERA)
    
    # Track peak
    if reward_per_block > peak_reward:
        peak_reward = reward_per_block
        peak_era = era
    
    rewards_per_era.append(reward_per_block)
    prev_supply = current_supply

# ═══════════════════════════════════════════════════════════════════════════
# OUTPUT STATISTICS
# ═══════════════════════════════════════════════════════════════════════════
print("📊 EMISSION STATISTICS:")
print(f"   - Total eras: {len(rewards_per_era)}")
print(f"   - Peak reward: {peak_reward / 10**18:.6f} SANC/block (Era {peak_era}, ~Year {peak_era * 24 / 24 / 365:.1f})")
print(f"   - Era 1 reward: {rewards_per_era[0] / 10**18:.10f} SANC/block")
print(f"   - Era {TOTAL_ERAS} reward: {rewards_per_era[-1] / 10**18:.10f} SANC/block")
print()

# Calculate total emission from table
total_from_table = sum(r * BLOCKS_PER_ERA for r in rewards_per_era)
print(f"   - Total emission from table: {total_from_table / 10**18:,.2f} SANC")
print(f"   - Max supply: {S_MAX / 10**18:,.2f} SANC")
print(f"   - Coverage: {total_from_table / S_MAX * 100:.4f}%")
print()

# ═══════════════════════════════════════════════════════════════════════════
# OUTPUT RUST CODE
# ═══════════════════════════════════════════════════════════════════════════
print("=" * 70)
print("// Copy this into runtime/src/constants.rs or pallets/emission/src/lib.rs")
print("=" * 70)
print()
print("/// Maximum supply of $SANC in smallest units (planck)")
print(f"pub const MAX_SUPPLY: u128 = {S_MAX};")
print()
print("/// Total number of eras in the emission schedule")
print(f"pub const TOTAL_ERAS: usize = {len(rewards_per_era)};")
print()
print("/// Blocks per era (24 hours at 6s block time)")
print(f"pub const BLOCKS_PER_ERA: u32 = {BLOCKS_PER_ERA};")
print()
print(f"/// Pre-computed block rewards per era (20 years, {len(rewards_per_era)} eras)")
print(f"/// Peak reward occurs at era {peak_era} (~year 10)")
print(f"pub const REWARD_SCHEDULE: [u128; {len(rewards_per_era)}] = [")

# Print in rows of 5 for readability
for i, r in enumerate(rewards_per_era):
    if i % 5 == 0:
        print("    ", end="")
    print(f"{r}_u128, ", end="")
    if (i + 1) % 5 == 0:
        print()  # Newline every 5 items

# Handle last line if not multiple of 5
if len(rewards_per_era) % 5 != 0:
    print()
    
print("];")
print()
print("// End of generated code")

# ═══════════════════════════════════════════════════════════════════════════
# SAVE TO FILE
# ═══════════════════════════════════════════════════════════════════════════
output_file = "emission_table.rs"
with open(output_file, "w") as f:
    f.write("// AUTO-GENERATED by scripts/generate_curve.py\n")
    f.write("// DO NOT EDIT MANUALLY\n")
    f.write("//\n")
    f.write("// Sanctuary Protocol v2.0 - Pre-computed Sigmoid Emission Table\n")
    f.write(f"// Max Supply: {S_MAX_UNITS:,} SANC\n")
    f.write(f"// Duration: {TOTAL_YEARS} years ({TOTAL_ERAS} eras)\n")
    f.write("//\n\n")
    
    f.write(f"/// Maximum supply of $SANC in smallest units (planck)\n")
    f.write(f"/// {S_MAX_UNITS:,} SANC × 10^18 = {S_MAX}\n")
    f.write(f"pub const MAX_SUPPLY: u128 = {S_MAX};\n\n")
    
    f.write(f"/// Total number of eras in the emission schedule\n")
    f.write(f"pub const TOTAL_ERAS: usize = {len(rewards_per_era)};\n\n")
    
    f.write(f"/// Blocks per era (24 hours at 6s block time)\n")
    f.write(f"pub const BLOCKS_PER_ERA: u32 = {BLOCKS_PER_ERA};\n\n")
    
    f.write(f"/// Pre-computed block rewards per era\n")
    f.write(f"/// Peak reward at era {peak_era} (~year 10): {peak_reward / 10**18:.6f} SANC/block\n")
    f.write(f"pub const REWARD_SCHEDULE: [u128; {len(rewards_per_era)}] = [\n")
    
    for i, r in enumerate(rewards_per_era):
        if i % 5 == 0:
            f.write("    ")
        f.write(f"{r}_u128, ")
        if (i + 1) % 5 == 0:
            f.write("\n")
    
    if len(rewards_per_era) % 5 != 0:
        f.write("\n")
        
    f.write("];\n")

print(f"\n✅ Emission table saved to: {output_file}")
print(f"   (Move to pallets/emission/src/emission_table.rs)")
