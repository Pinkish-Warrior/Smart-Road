# Traffic Simulation: Smart Intersection Management - Step-by-Step Plan

This document outlines a structured approach to building the **Autonomous Vehicle (AV) traffic simulation project**.  
Each step builds upon the last, creating a logical and manageable workflow from setup to completion.

---

## Step 1: Project Setup & Basic Window

**Goal:** Initialize the Rust project and get a basic, blank window running with SDL2.

### 1.1 Initialize Project

```bash
cargo new smart_intersection
cd smart_intersection
```

### 1.2 Add Dependencies

Open `Cargo.toml` and add:

```toml
[dependencies]
sdl2 = "0.35"
rand = "0.8"
```

### 1.3 Create the Main Window

In `src/main.rs`, initialize SDL2, create a window with a canvas, and implement a basic game loop that waits for a quit event.  
The window should open, display a solid color, and close without errors — confirming your setup works.

---

## Step 2: World Modeling & Asset Loading

**Goal:** Define the structure of the intersection and render the static background.

### 2.1 Load Assets

- Create an `assets` directory in your project root.  
- Download or create a road texture for the cross-intersection and place it there.  
- Load this image into an SDL2 texture in your code.

### 2.2 Define the Coordinate System & Routes

- Use a 1200x1200 pixel world.
- Mathematically define 12 lanes and their paths (lines or Bézier curves).
- Create structs or enums for:
  - `Lane`
  - `Direction` (North, South, East, West)
  - `Route` (Right, Straight, Left)

**Example Path Definition:**  
A vehicle turning right from the West might follow an arc from `(0, 550)` to `(650, 0)`.

### 2.3 Render the World

In the game loop, clear the canvas and render the road texture as the background.

---

## Step 3: Vehicle Structuring & Basic Physics

**Goal:** Create the `Vehicle` struct and implement movement logic.

### 3.1 Create the Vehicle Struct

```rust
struct Vehicle {
    id: u32,
    position: (f64, f64),
    velocity: f64,
    route: Route,
    texture: Texture,
    rotation: f64,
    time_in_intersection: f64,
}
```

### 3.2 Implement Vehicle Movement

Add:

```rust
fn update(&mut self, delta_time: f64) {
    // update position based on velocity and path
}
```

Rotation changes dynamically along curved paths.

### 3.3 Implement Safety Distance Check

Check distance to the vehicle ahead:

```rust
if distance < SAFE_DISTANCE {
    self.velocity = front_vehicle.velocity;
}
```

---

## Step 4: Smart Intersection Management Algorithm

**Goal:** Design and implement core collision avoidance and traffic flow logic.

### 4.1 Create the `IntersectionManager` Struct

- Tracks all vehicles approaching or in the intersection.
- Keeps a record of reserved paths in time and space.

### 4.2 Design the Control Strategy (Reservation-Based)

- **Communication Zone:** Vehicles send route requests upon entering.
- **Request:** Vehicle sends its route.
- **Conflict Check:** Compare projected space-time paths for conflicts.
- **Decision:**
  - **No conflict:** Permission granted.
  - **Conflict:** Vehicle adjusts velocity and retries.

### 4.3 Implement Logic

Functions:

```rust
fn request_entry()
fn check_for_conflicts()
fn grant_permission()
```

---

## Step 5: Animation & Full Rendering

**Goal:** Animate vehicles correctly.

### 5.1 Load Vehicle Assets

Load sprite textures for different directions.

### 5.2 Animate Turning

Use:

```rust
canvas.copy_ex(&texture, src_rect, dest_rect, rotation, None, false, false)?;
```

---

## Step 6: User Commands & Vehicle Generation

**Goal:** Implement controls to spawn vehicles and manage the simulation.

### 6.1 Event Handling

Extend your main event loop to listen for:

```rust
Event::KeyDown { .. }
```

### 6.2 Implement Spawning Logic

- **Arrow Keys:** Spawn vehicles from corresponding directions.
- **‘R’ Key:** Toggle random continuous spawning.
- **Non-overlapping Spawn:** Skip creation if spawn point occupied.

### 6.3 Exit Command

Press `Esc` to end the simulation and show statistics.

---

## Step 7: Statistics Collection & Display

**Goal:** Track simulation data and display in a new window.

### 7.1 Create a Statistics Struct

```rust
struct Statistics {
    max_vehicles_passed: u32,
    max_velocity: f64,
    min_velocity: f64,
    max_time: f64,
    min_time: f64,
    close_calls: u32,
}
```

### 7.2 Collect Data

- Update stats when vehicles exit intersection.
- Track `close_calls` when distance < `SAFE_DISTANCE`.

### 7.3 Display Statistics

Use SDL2’s `ttf` module to display data in a results window.

---

## Step 8: Bonus Features & Refinement

**Goal:** Polish and enhance the project.

### 8.1 Acceleration & Deceleration

Add:

```rust
velocity += acceleration * delta_time;
```

Ensure it doesn’t exceed `max_speed`.

### 8.2 Additional Statistics

Track:

- Average wait time
- Throughput (vehicles/minute)

### 8.3 Code Cleanup & Documentation

- Comment complex logic.  
- Add a detailed `README.md` explaining setup and controls.

---
