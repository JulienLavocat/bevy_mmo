## Random notes

### Spatial partitioning optimizations

When spatial partitioning will be completed (spatial hash grid) one optimization is to compute the packet for each grid cell.
When generating the world state snapshot, we iterate over all player entities:

1. Check if player is in a cell that has already been generated, if yes send cached snapshot and return
2. If not, generate the snapshot, cache it and send it

Example :

- Player A is in cell **1** -> Snapshot not cached, generate a snapshot for cell **1** and store it
- Player B is in cell **2** -> Snapshot not cached, generate a snapshot for cell **2** and store it
- Player C is in cell **2** -> Found cached snapshot for cell **2**, send it directly to player C
