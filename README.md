# No Name (YET)

## Overview
A 2D platformer roguelike game with RPG elements, procedural generation, multiple characters with skill trees, and meta-progression.

> This game is heavily inspired by Vagante.

## Development Roadmap

This roadmap breaks the development into phases with detailed steps.

### Phase 1: Core Setup and Game Loop

- [ ] Initialize the project and dependencies.
- [ ] Implement the game loop with delta-time tracking.
- [ ] Set up window management and input handling using winit.
- [ ] Implement basic input events (e.g., ESC to quit).
- [ ] Create an app context to share app state.

### Phase 2: Entity-Component System (Component Pattern) and Signals

- [ ] Implement a signal/event system to decouple components.
- [ ] Implement an Entity struct to manage components and events.
- [ ] Implement a Component trait with lifecycles so that user defined components can implement.
- [ ] Add a flat array for entities to the game loop and call components lifecycles.

### Phase 3: Transform System

- [ ] Create a Transform struct to manage position, rotation, and scale.
- [ ] Implement matrix transformations for efficient screen-world projection.
- [ ] Add utility functions to convert world coordinates to screen space.
- [ ] Create a PositionComponent implementing the Component trait.

### Phase 4: Scene Manager

- [ ] Implement a SceneManager to handle multiple scenes (menus, gameplay, etc.).
- [ ] Use scenes for transitions (title screen → game level → victory screen).
- [ ] Ensure the SceneManager supports scene transitions with smooth loading.

### Phase 5: Physics System and Entities

- [ ] Add matrix-based operations for movement and collision detection.
- [ ] Implement immovable entities (e.g., walls, platforms).
- [ ] Implement movable entities (e.g., player, enemies, bosses).

### Phase 6: Combat, Inventory, and RPG Elements

- [ ] Implement combat mechanics (attacks, damage, cooldowns).
- [ ] Create an inventory system for equipping weapons, armor, and rings.
- [ ] Implement player and enemy stats with dynamic effects from items.

### Phase 7: Procedural Level Generation

- [ ] Implement random room generation (e.g., cellular automata).
- [ ] Add loot generation logic for random items with stats.
- [ ] Ensure generated rooms contain loot, traps, and enemies.

### Phase 8: Character Skill Trees and Meta-Progression

- [ ] Implement skill unlocking and character upgrades.
- [ ] Create multiple characters with unique skill trees.
- [ ] Add meta-progression mechanics where characters ascend after a successful run.

### Phase 9: Visuals and UI

- [ ] Render vertex with textures using wgpu.
- [ ] Implement an Animation System for entities.
- [ ] Add a simple UI for the HUD, inventory, and menus.

### Phase 10: Sounds and Music

- [ ] Add sound effects for actions (e.g., attack, jump).
- [ ] Implement background music for different scenes (title screen, gameplay).

### Phase 11: Polishing and Release

- [ ] Optimize the game for smooth performance at 60 FPS.
- [ ] Playtest for bugs and balancing issues.
- [ ] Prepare the game for distribution (packaging executables).