# GSG Endpoint CLI

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
          - fancy: Fancy version which uses Discord emojis available in the main DRG Discord server

  -h, --help
          Print help (see a summary with '-h')
```

### Example

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

+-------+-------------------+----------------------+----------------+-------------------+
| Stage | Primary           | Secondary            | Warning        | Mutator           |
+-------+-------------------+----------------------+----------------+-------------------+
| 1     | Mining Expedition | Kill 1 Dreadnought   | Parasites      |                   |
+-------+-------------------+----------------------+----------------+-------------------+
| 2     | Egg Hunt          | Repair 2 Mini-mules  | Mactera Plague |                   |
+-------+-------------------+----------------------+----------------+-------------------+
| 3     | Salvage Operation | Perform 2 Deep Scans |                | Critical Weakness |
+-------+-------------------+----------------------+----------------+-------------------+


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

## Known limitations

- Does not try to determine exact Dreadnought kind and order.
- Does not show exact primary objective quantity (pending improvement).
