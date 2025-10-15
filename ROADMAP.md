
# Smart Intersection Simulation: Step-by-Step Guide

This guide breaks down the project into logical phases, from setting up the environment to implementing the core algorithm and final statistics.

## Phase 1: Project Setup & Basic Window

**Goal:** Get a basic window running and render a static background. This ensures your development environment and graphics library are correctly configured.

1. **Setup Rust Environment:**

      * Ensure you have Rust and Cargo installed (`rustup --version`, `cargo --version`).
      * Create a new Cargo project: `cargo new smart_intersection`
      * Move into the project directory: `cd smart_intersection`

2. **Add Dependencies:**

      * Open your `Cargo.toml` file and add the `sdl2` crate for graphics, windowing, and event handling.

        ```toml
        [dependencies]
        sdl2 = "0.36.0"
        ```

3. **Create the Game Window:**

      * In `src/main.rs`, write the initial code to initialize the SDL2 library.
      * Create a window with a title (e.g., "Smart Intersection") and a canvas for rendering.

4. **Implement the Game Loop:**

      * Create a main loop (`'running: loop {}`) that will handle events and redraw the screen on each frame.
      * Handle the `Quit` event so you can close the window.

5. **Render the Background:**

      * Find or create a simple image for the cross-intersection.
      * Load this image as a texture and render it to the canvas on every frame. Your window should now display your intersection.

-----

## Phase 2: Vehicle Struct & Basic Movement

**Goal:** Define the vehicle's properties and get a single vehicle moving in a straight line across the screen.

1. **Create the `Vehicle` Struct:**

      * Define a struct to hold all the necessary information about a vehicle.

        ```rust
        struct Vehicle {
            x: f64,
            y: f64,
            velocity: f64,
            route: Route, // An enum for the path
            texture: Texture, // The vehicle's image
            // ... other fields like dimensions, time trackers
        }
        ```

2. **Define Routes:**

      * Create an `enum` to represent all 12 possible routes (e.g., `SouthToNorth_Right`, `WestToEast_Straight`, etc.).

3. **Implement Movement Logic:**

      * Create a collection to hold all active vehicles (e.g., `Vec<Vehicle>`).
      * In your game loop, iterate through each vehicle and update its `x` and `y` coordinates based on its `velocity` and `route`.
      * For now, just implement one simple straight path (e.g., west to east).
      * Render the vehicle's texture at its new position in each frame. You should see a car moving across the screen.

-----

## Phase 3: User Input & Vehicle Spawning

**Goal:** Allow the user to generate vehicles dynamically using the keyboard.

1. **Handle Keyboard Events:**

      * Expand your event handling logic within the game loop to listen for `KeyDown` events.

2. **Implement Spawning Logic:**

      * **Arrow Keys:** When an arrow key is pressed, create a new `Vehicle` instance. Set its starting position, velocity, and a random route (r, s, or l) based on the key pressed.
          * `ArrowUp`: Spawns at the bottom (South), moving North.
          * `ArrowDown`: Spawns at the top (North), moving South.
      * **'R' Key:** Create a boolean flag that is toggled by the 'R' key. When `true`, generate a random vehicle from a random direction in each frame (or on a timer).
      * **Spawning Safety:** Before creating a new vehicle, check if the spawn area is clear. A simple way is to check the distance from the last car spawned in that lane. If it's too close, don't spawn a new one.

-----

## Phase 4: The Core Algorithm - Intersection Management

**Goal:** Design and implement the "smart" logic that allows vehicles to cross the intersection without colliding. A reservation-based system is a great approach.

1. **Define the Intersection Zone:**

      * Mathematically define the central intersection area as a rectangle. A vehicle is "in the intersection" when its coordinates are within this box.

2. **Identify Conflict Zones:**

      * Map out the smaller zones within the intersection where paths can cross. There are a finite number of these collision points. For example, a car from the South going straight conflicts with a car from the West turning left.

3. **Create the `IntersectionManager`:**

      * This will be the "brain" of your system. Create a new struct to manage traffic.
      * It should have a queue for vehicles waiting to enter and a schedule/log of reservations for the conflict zones.

4. **Implement the Reservation Protocol:**

      * **Request:** When a vehicle reaches the boundary of the intersection, it sends a request to the `IntersectionManager`.
      * **Check:** The manager calculates the path the vehicle will take and which conflict zones it will occupy, and for how long (based on its velocity). It checks if these time slots for those zones are already reserved.
      * **Grant/Deny:**
          * If the path is clear, the manager grants permission, and the vehicle proceeds. The manager reserves those time slots.
          * If the path is *not* clear, the manager denies the request. It can either tell the vehicle to stop before the intersection or give it a slower speed to arrive when the path is clear.

-----

## Phase 5: Animation & Visual Polish

**Goal:** Make the simulation look more realistic by animating vehicle movement.

1. **Load Vehicle Sprites:**

      * Use the asset links provided or create your own. You'll want separate images for the car facing North, East, South, and West.

2. **Implement Rotation:**

      * The key is to change the vehicle's sprite or rotation angle as it moves.
      * For a turn, instead of a sharp 90-degree snap, smoothly interpolate the rotation angle over several frames as it travels along the curved path. You can pre-calculate the points on the arc of the turn.

-----

## Phase 6: Statistics Tracking & Display

**Goal:** Collect data during the simulation and display it at the end.

1. **Create a `Statistics` Struct:**

      * Create a struct to hold all the required data: `total_vehicles_passed`, `max_velocity`, `min_velocity`, `max_time`, `min_time`, `close_calls`.

2. **Collect Data:**

      * **Time:** Start a timer for a vehicle when it enters the control zone and stop it when it exits. Use this to update `max_time` and `min_time`.
      * **Velocity:** In every frame, check each car's velocity against the `max_velocity` and `min_velocity` stored in your stats struct and update if necessary.
      * **Close Calls:** In the game loop, check the distance between all pairs of cars inside the intersection. If the distance is less than your defined safety distance, increment the `close_calls` counter.
      * **Passed Vehicles:** Increment the counter when a vehicle is successfully removed after crossing.

3. **Implement End Screen:**

      * When the `Esc` key is pressed, break the main loop.
      * After the loop, create a new, simple window.
      * Use SDL2's font rendering capabilities (`sdl2::ttf`) to draw the final statistics as text onto this new window.

-----

## Phase 7: Bonus Features (Optional)

**Goal:** Enhance the simulation with more advanced physics and features.

1. **Acceleration & Deceleration:**
      * Add `acceleration` and `target_velocity` fields to your `Vehicle` struct.
      * Instead of setting velocity instantly, have the `IntersectionManager` set a `target_velocity`.
      * In each frame, update the vehicle's current velocity towards the target velocity based on its acceleration rate.
2. **Add More Statistics:**
      * Track and display useful metrics like average wait time per vehicle or intersection throughput (vehicles per minute).
