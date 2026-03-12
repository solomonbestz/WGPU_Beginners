```markdown
# WGPU Beginners – Rust GPU Rendering Setup

## Overview

This project is a **beginner-friendly exploration of GPU programming in Rust** using:

- **wgpu** – modern cross-platform GPU API
- **glfw** – window creation and event handling

The goal of this project is to **learn how graphics pipelines work from the ground up**.

Instead of using a full game engine, we build the **core rendering system ourselves**, understanding how the CPU communicates with the GPU.

---

# What This Project Teaches

This project demonstrates how to:

- Create a window using GLFW
- Initialize the GPU using WGPU
- Connect a window to a GPU rendering surface
- Select a GPU device
- Configure a rendering surface
- Prepare the system for drawing frames

This is the **foundation for building a renderer or game engine**.

---

# Final Goal of This Project

By the end of this project, we will build a small rendering engine capable of:

- Rendering triangles
- Drawing 3D objects
- Running shaders
- Displaying animated frames
- Building a real-time render loop

Ultimately this will evolve into a **basic GPU rendering engine written in Rust**.

---

# Graphics Pipeline Mental Model

A modern GPU renderer works like a **factory pipeline**:

```

Vertices
↓
Vertex Shader
↓
Triangle Assembly
↓
Rasterization
↓
Fragment Shader
↓
Framebuffer
↓
Surface
↓
Window
↓
Monitor

```

Every frame of graphics goes through this process.

---

# Core Concepts

## Window

A **window** is created by the operating system and managed by GLFW.

It represents the application interface:

```

Desktop
└ Window
└ drawing area

```

The GPU **cannot render directly to the window**.

---

## Surface

A **surface** connects the GPU renderer to the window.

```

GPU → Surface → Window → Screen

```

The surface is where rendered images are presented.

Think of it as the **screen inside the window**.

---

## Instance

The **wgpu instance** is the entry point into the graphics system.

It connects your program to platform graphics APIs such as:

- Vulkan
- Metal
- DirectX
- OpenGL

---

## Adapter

An **adapter** represents a physical GPU.

Examples:

- NVIDIA GPU
- AMD GPU
- Intel integrated graphics

The program asks the system:

> Which GPU should be used for rendering?

---

## Device

The **device** is the main interface used to communicate with the GPU.

It is responsible for creating:

- buffers
- textures
- shaders
- pipelines

---

## Queue

The **queue** sends commands to the GPU.

Example commands:

```

draw triangle
update buffer
dispatch shader

```

Commands are submitted to the GPU through the queue.

---

## Surface Configuration

The **surface configuration** defines how frames are displayed.

Important settings include:

- resolution
- pixel format
- presentation mode
- frame buffering

---

# Frames

A **frame** is a single rendered image.

Games and graphics applications display many frames per second.

Example:

```

60 FPS = 60 frames per second

```

Each frame is rendered independently by the GPU.

---

# Framebuffer

The **framebuffer** stores the pixel data for a frame.

Example resolution:

```

1920 × 1080

```

That means the framebuffer stores:

```

2,073,600 pixels

```

Each pixel contains color information.

---

# Double Buffering

Graphics systems typically use **two framebuffers**:

```

Front Buffer → currently displayed
Back Buffer  → GPU rendering next frame

```

When rendering finishes, the buffers swap.

```

Back → Front
Front → Back

```

This prevents flickering and tearing.

---

# Vertex

A **vertex** is a point in 2D or 3D space.

Example:

```

(x, y, z)

```

Triangles are built from vertices.

```

A
|
| 
C--B

```

All 3D models are constructed from triangles.

---

# Vertex Buffer

A **vertex buffer** stores vertex data inside GPU memory.

Example structure:

```

VertexBuffer
├ vertex1
├ vertex2
├ vertex3

```

The GPU reads vertices from this buffer to build shapes.

---

# Rendering Pipeline

The **render pipeline** describes how vertices become pixels.

```

Vertex Data
↓
Vertex Shader
↓
Triangle Assembly
↓
Rasterization
↓
Fragment Shader
↓
Framebuffer

```

This pipeline runs **every frame**.

---

# CPU vs GPU

Your Rust program runs on the **CPU**.

The GPU is a separate processor specialized for graphics.

Communication flow:

```

CPU (Rust code)
↓
Command Queue
↓
GPU
↓
Frame buffer
↓
Surface
↓
Window
↓
Screen

```

The CPU sends instructions, and the GPU executes them.

---

# Project Structure

```

src/
└ main.rs

```

The `State` struct stores all GPU resources.

```

State
├ instance
├ surface
├ device
├ queue
├ configuration
├ window size
└ window reference

```

This structure represents the **graphics context of the application**.

---

# Event Loop

The main program runs a loop that processes window events.

```

while window is open
poll events
handle input
swap buffers

```

This will later evolve into a **render loop**.

---

# Future Steps

Next steps in this project include:

1. Configure the surface
2. Create the render pipeline
3. Write shaders
4. Create vertex buffers
5. Draw the first triangle
6. Implement a render loop

---

# Technologies Used

- Rust
- GLFW
- WGPU

---

# Long-Term Vision

This project aims to build a **small graphics renderer** capable of:

- real-time rendering
- GPU pipelines
- 3D graphics
- shader programming

It is designed as a **learning project to understand modern GPU architecture**.

---

# Learning Outcomes

By completing this project you will understand:

- modern GPU rendering pipelines
- graphics API architecture
- framebuffers and surfaces
- vertex buffers and shaders
- how real-time graphics engines work

This knowledge applies directly to modern graphics engines such as:

- Unreal Engine
- Unity
- Godot
- custom rendering engines
```