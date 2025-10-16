# Smart Road - Quick Start Guide

## 🚗 Welcome to Smart Road!
An autonomous vehicle intersection simulator with intelligent traffic management.

## 🎮 Controls at a Glance

### Spawn Vehicles
| Key | Color | Direction | Route |
|-----|-------|-----------|-------|
| ↑   | 🔴 Red | South → North | Random turn at intersection |
| ↓   | 🟢 Green | North → South | Random turn at intersection |
| ←   | 🟡 Yellow | East → West | Random turn at intersection |
| →   | 🔵 Blue | West → East | Random turn at intersection |

### Special Controls
- **R** - Random Mode (continuously spawns random vehicles)
- **H** - Toggle Help Panel (detailed control info)
- **F** - Fullscreen Mode
- **ESC** - Exit & Show Statistics

## 📊 Understanding the HUD

### Top Bar Indicators (Row 1)
Each indicator shows a colored square, label, and real-time number:
- 🔴 **ACTIVE** - Number of cars currently on screen
- 🟢 **PASSED** - Total cars that completed their journey
- 🟡 **YIELDS** - Times vehicles had to slow down/stop
- 🟠 **CLOSE** - Near-collision incidents (safety violations)

### Velocity Bars (Row 2)
- 🔵 **MIN VEL** - Blue bar showing slowest speed + value in px/s
- 🔴 **MAX VEL** - Red bar showing fastest speed + value in px/s

### FPS Meter (Top-Right Corner)
- **Label**: "FPS" with numeric value
- **Green Bar**: Good performance (≥50 fps)
- **Yellow Bar**: OK performance (30-49 fps)
- **Red Bar**: Poor performance (<30 fps)

## 🎯 Game Objectives

### Smart Intersection Management
- Vehicles automatically navigate the intersection
- Cars detect other vehicles and maintain safe distances
- Speed adjustments prevent collisions
- Smooth turns and lane changes

### Statistics Tracking
The system monitors:
- Total cars passed through intersection
- Number of times vehicles had to yield ("give ways")
- Close calls (safety distance violations)
- Fastest and slowest vehicle speeds
- Time taken to cross intersection (max/min)

## 🌟 Visual Features

### Animation Effects
- **Speed Trails**: Fast-moving cars leave colored trails
- **Glow Effect**: High-speed vehicles have yellow glow
- **Smooth Turning**: Cars rotate smoothly when changing direction
- **Acceleration**: Speed changes gradually (realistic physics)

### Environment
- **Trees**: Procedurally generated with depth and shadows
- **Roads**: Detailed lane markings and intersection zones
- **Background**: Textured with subtle color variations
- **Cars**: 3D-styled vehicles with windows, lights, and mirrors

## 📈 Safety Ratings

When you exit (ESC), you'll see a safety rating:
- **✓ EXCELLENT**: No close calls - perfect traffic management!
- **⚠ GOOD**: Few close calls (< 5) - decent performance
- **✗ NEEDS IMPROVEMENT**: Many close calls (≥ 5) - too aggressive

## 💡 Tips for Best Results

1. **Start Slow**: Use arrow keys to spawn individual cars and observe behavior
2. **Test Directions**: Try different spawn directions to see varied routes
3. **Watch for Patterns**: Notice how cars automatically yield to avoid collisions
4. **Random Mode**: Press R for continuous action (can get chaotic!)
5. **Check Stats**: Press ESC periodically to see your management statistics

## 🎨 Color Coding

### Car Colors = Spawn Direction
- 🔴 **Red**: Coming from South (bottom)
- 🟢 **Green**: Coming from North (top)
- 🟡 **Yellow**: Coming from East (left)
- 🔵 **Blue**: Coming from West (right)

### Speed Indicators
- **Fast cars** (> 80% max speed): Yellow glow + long trails
- **Medium cars**: Moderate trails
- **Slow cars**: Minimal trails

## ⚙️ Technical Details

### Vehicle Behavior
- **3 Speed Levels**: Fast, Default, Slow
- **Safety Distance**: 1.5× lane width
- **Smart Yielding**: Cars slow/stop to avoid collisions
- **Route Selection**: Random turns (left, straight, right)

### Performance
- **Target FPS**: 60
- **Frame Time**: 16ms
- **Keypress Interval**: 128ms (prevents spam)

## 🏆 Challenge Yourself!

### Easy Mode
- Spawn cars one at a time
- Let them clear before spawning more
- Goal: 0 close calls

### Medium Mode
- Use random mode (R key)
- Try to get 20+ cars through safely
- Goal: < 3 close calls

### Hard Mode
- Spam multiple spawn keys rapidly
- Create maximum traffic density
- Goal: System still manages without crashes!

## 📝 Exit Statistics

When you press ESC, you'll see:
```
=== SMART ROAD STATISTICS ===

Traffic Summary:
• Cars passed: [number]
• Give ways: [number]
• Close calls: [number]

Velocity Stats:
• Max velocity: [number]px/s
• Min velocity: [number]px/s

Time Stats:
• Max time: [number]s
• Min time: [number]s

Safety Rating: [rating]
```

## 🐛 Troubleshooting

- **Low FPS?** Close other applications, try non-fullscreen mode
- **No cars appearing?** Make sure you're pressing arrow keys or R
- **Cars stuck?** This shouldn't happen - the system handles all conflicts
- **Help not showing?** Press H key

## 🎓 Project Background

This simulator is based on the Rust Piscine `road_intersection` project, implementing:
- Autonomous vehicle intersection management
- Smart traffic control without traffic lights
- Real-time physics and collision avoidance
- Comprehensive statistics tracking
- Enhanced UI/UX for better understanding

---

**Enjoy managing your smart intersection! 🚦🚗💨**

Press **H** in-game for visual control guide!
