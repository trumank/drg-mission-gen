# GSG Endpoint CLI

Last reversed engineered and tested against game version:

```
S05.04.1 (102179 - 2/07/2024)
```

## Usage

```
Usage: drg_mission_gen_gsg_endpoint_cli.exe [OPTIONS]

Options:
  -f, --format <FORMAT>
          What do you want the output format to be

          [default: json]

          Possible values:
          - json
          - plain: Simple human-friendly table format
          - discord: Discord version which uses Discord emojis available in the main DRG Discord server

  -h, --help
          Print help (see a summary with '-h')
```

### Example: `--format=plain`

```bash
$ cargo run --bin drg_mission_gen_gsg_endpoint_cli -- --format=plain
```

Example `--format=plain` output:

```
=== Deep Dive Info ===
Start: 2024-07-11
End: 2024-07-18
Seed: 3422115630

=== Normal Deep Dive ===
Codename: Unknown Comeback
Biome: Azure Weald

+-------+--------------+----------------------+----------------+-------------------+
| Stage | Primary      | Secondary            | Warning        | Mutator           |
+-------+--------------+----------------------+----------------+-------------------+
| 1     | 250 Morkite  | Kill 1 Dreadnought   | Parasites      |                   |
+-------+--------------+----------------------+----------------+-------------------+
| 2     | 6 Eggs       | Repair 2 Mini-mules  | Mactera Plague |                   |
+-------+--------------+----------------------+----------------+-------------------+
| 3     | 3 Mini-mules | Perform 2 Deep Scans |                | Critical Weakness |
+-------+--------------+----------------------+----------------+-------------------+


=== Elite Deep Dive ===
Codename: Clean Bed
Biome: Sandblasted Corridors

+-------+------------------+-----------------------+----------------+-----------------+
| Stage | Primary          | Secondary             | Warning        | Mutator         |
+-------+------------------+-----------------------+----------------+-----------------+
| 1     | On-Site Refinery | Perform 2 Deep Scans  | Duck and Cover |                 |
+-------+------------------+-----------------------+----------------+-----------------+
| 2     | Escort Duty      | Refine Liquid Morkite | Swarmageddon   |                 |
+-------+------------------+-----------------------+----------------+-----------------+
| 3     | On-Site Refinery | Collect 150 Morkite   | Lethal Enemies | Rich Atmosphere |
+-------+------------------+-----------------------+----------------+-----------------+
```

### Example: `--format=discord`

This tries to preserve the typical formatting used in the #deep-dive-discussions weekly deep dive
info post in the main DRG discord.

```bash
$ cargo run --bin drg_mission_gen_gsg_endpoint_cli -- --format=discord
```

Example `--format=discord` output:

```
Weekly Deep Dives information for **2024-07-11 to 2024-07-18**.
Deep Dives will reset **<t:1721300400:F>** (<t:1721300400:R>)

:Deep_Dive: __**DEEP DIVE**__ :Deep_Dive:
Region: **Azure Weald** | Code Name: **Unknown Comeback**
Stage 1: **:morkite: 250 Morkite** + **:dreadegg: Dreadnought** | :tothebone: **Parasites**
Stage 2: **:gegg: 6 Eggs** + **:molly: Repair 2 Mini-mules** | :tothebone: **Mactera Plague**
Stage 3: **:molly: 3 Mini-mules** + **:pingdrg: Perform 2 Deep Scans** | :rocknstone: **Critical Weakness**

:Deep_Dive: __**ELITE DEEP DIVE**__ :Deep_Dive:
Region: **Sandblasted Corridors** | Code Name: **Clean Bed**
Stage 1: **:refinerywell: On-Site Refinery** + **:pingdrg: Perform 2 Deep Scans** | :tothebone: **Duck and Cover**
Stage 2: **:drill: Escort Duty** + **:refinerywell: Connect 1 Pumpjack** | :tothebone: **Swarmageddon**
Stage 3: **:refinerywell: On-Site Refinery** + **:morkite: 150 Morkite** | :rocknstone: **Rich Atmosphere** :tothebone: **Lethal Enemies**
```

## Known limitations

- Does not try to determine exact Dreadnought kind and order.
- Does not show exact primary objective quantity (pending improvement).
