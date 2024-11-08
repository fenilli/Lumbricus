# Pong Rust - Game Design Document

Classic Pong Clone is a single-player, faithful recreation of the original Pong. The player controls a paddle on the left side of the screen, aiming to hit a bouncing ball past an AI-controlled paddle on the right.

---

## Table of Contents

1. [Game Overview](#game-overview)
2. [Gameplay](#gameplay)
3. [Core Mechanics](#core-mechanics)
4. [Graphics & Visuals](#graphics--visuals)
5. [Technical Requirements](#technical-requirements)
6. [Development Roadmap](#development-roadmap)

---

## 1. Game Overview

- **Title**: Pong Rust
- **Genre**: Arcade
- **Platform**: PC
- **Engine**: Custom-built with Rust ([`winit`](https://github.com/rust-windowing/winit) for window management, [`wgpu`](https://github.com/gfx-rs/wgpu/) for rendering)
- **Target Audience**: Casual gamers, fans of retro-style arcade games

## 2. Gameplay

- **Objective**: Control the paddle to prevent the ball from passing your side of the screen. Score a point each time the ball passes the AI paddle.
- **Victory Condition**: Score a predefined number of points (e.g., 10) to win the match.
- **Losing Condition**: If the AI scores the predefined number of points first, the player loses.
- **Game Flow**:
  - **Start Screen**: "Start Game" button to begin gameplay.
  - **Gameplay Loop**: The player moves their paddle up and down to return the ball, which bounces off boundaries and paddles. The AI paddle simply follows the ball's vertical position.
  - **Scoring**: Basic counter for player and AI scores displayed at the top.
  - **End Game**:
    - Display "Victory" or "Game Over" based on the match outcome.
    - Option to restart the game or return to the main menu.

## 3. Core Mechanics

- **Paddle Movement**:
  - **Player Paddle**: Moves vertically using `W` and `S` keys (or arrow keys).
  - **AI Paddle**: Follows the ball’s vertical position, moving at a steady speed.

- **Ball Physics**:
  - The ball bounces at a fixed angle and speed, reversing direction upon hitting paddles or screen boundaries.

- **Scoring**:
  - The player scores by getting the ball past the AI paddle.
  - The AI scores by getting the ball past the player’s paddle.

## 4. Graphics & Visuals

- **Art Style**: Pure geometry with basic shapes; monochrome colors (white on black background) for a retro look.
- **User Interface**:
  - Simple score display at the top of the screen.
  - Minimalistic buttons for menu interactions.

## 5. Technical Requirements

- **Rendering**: `wgpu` for basic graphics rendering.
- **Input Handling**: Keyboard input for player paddle movement.
- **Game State Management**: Simple loop to handle `MainMenu`, `Playing`, and `GameOver` states.

## 6. Development Roadmap

**Phase 1**: Implement Basic Gameplay Loop  
  - Paddle and ball movement, collision detection, and scoring system.

**Phase 2**: Add User Interface  
  - Score display, start screen, and end screen.