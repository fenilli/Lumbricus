# Lumbricus

## Overview

A 2D platformer roguelike game with RPG elements, procedural generation, multiple characters with skill trees, and meta-progression.

> This game is heavily inspired by Vagante.

## Development Roadmap

This roadmap breaks the development into phases with detailed steps.

### Phase 1: Core Setup and Game Loop

- [x] Initialize the project and dependencies.
- [x] Set up and engine struct for window management using winit.
- [x] Implement the game loop with delta-time tracking.
- [x] Implement input event handling (e.g., ESC to quit).

### Phase 2: Graphics and Rendering

- [ ] Create the render state for surface initialization and command encoder.
- [ ] Create render pipeline to enable shaders.
- [ ] Implement a mesh struct for vertex and index buffers.
- [ ] Implement texture loading and add binding groups to drawing.
- [ ] Create a uniform buffer for projection.
- [ ] Draw something to the screen.
- [ ] Implement an Animation System.
- [ ] Add a simple UI for the HUD, inventory, and menus.

### Phase 3: Scene Manager

- [ ] Implement a SceneManager to handle multiple scenes (menus, gameplay, etc.).
- [ ] Use scenes for transitions (title screen → game level → victory screen).
- [ ] Implement layers to the scene, so the order of drawing is correct.

### Phase 4: Transform System

- [ ] Create a Transform struct to manage position, rotation, and scale.
- [ ] Implement matrix transformations for efficient screen-world projection.
- [ ] Add utility functions to convert world coordinates to screen space.

### Phase 5: Entity-Component System (Component Pattern) and Signals

- [ ] Implement a signal/event system to decouple components.
- [ ] Implement an Entity struct to manage components and events.
- [ ] Implement a Component trait with lifecycles so that user defined components can implement.
- [ ] Add a flat array for entities to the game loop and call components lifecycles.

### Phase 6: Physics System and Gameplay

- [ ] Add matrix-based operations for movement and collision detection.
- [ ] Implement immovable entities (e.g., walls, platforms).
- [ ] Implement movable entities (e.g., player, enemies, bosses).
- [ ] Implement combat mechanics (attacks, damage, cooldowns).

### Phase 7: Polishing and Release

- [ ] Optimize the game for smooth performance at least 60 FPS.
- [ ] Playtest for bugs and balancing issues.
- [ ] Give it a real product name.
- [ ] Prepare the game for distribution (packaging executables).

## Optional Features

These are features that will be worked after the core features are implemented and a small prototype game can be played.

### Phase 1: Procedural Level Generation

- [ ] Implement random room generation (e.g., cellular automata).
- [ ] Add loot generation logic for random items with stats.
- [ ] Ensure generated rooms contain loot, traps, and enemies.

### Phase 2: Character's Inventory, Skill Trees and Meta-Progression

- [ ] Create an inventory system for equipping weapons, armor, and rings.
- [ ] Implement player and enemy stats with dynamic effects from items.
- [ ] Implement skill unlocking and character upgrades.
- [ ] Create multiple characters with unique skill trees.
- [ ] Add meta-progression mechanics where characters ascend after a successful run.

### Phase 3: Sounds and Music

- [ ] Add sound effects for actions (e.g., attack, jump).
- [ ] Implement background music for different scenes (title screen, gameplay).
