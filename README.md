# Rust Barrier - Share Your Mouse and Keyboard

## What It Does
- Controls multiple computers with one keyboard and mouse
- Moves smoothly between screens like they're one big display
- Works on Linux and Windows
- Uses your network to send mouse and keyboard signals

## How It Works

### 1. Events
We track three main things:
- Mouse movements (where your cursor goes)
- Keyboard actions (which keys you press)
- Screen switches (when you move to another computer)

### 2. Network
- Uses TCP to send events between computers
- One computer acts as the server (where your real mouse/keyboard are)
- Other computers connect as clients
- Everything happens instantly over your local network

### 3. Platform Support
- Works with Linux (X11) and Windows
- Converts keyboard/mouse signals to work on each system
- Same experience on all computers

## Testing
We test four main things:
1. Basic Connectivity
   - Can computers connect?
   - Do events arrive correctly?

2. Mouse Events
   - Track cursor position
   - Handle mouse clicks

3. Keyboard Events
   - Capture key presses
   - Send them to right computer

4. Screen Switching
   - Move between computers
   - Keep track of which screen is active

## Why These Choices?
- TCP: Reliable and simple (better than UDP for this)
- JSON: Easy to debug and understand
- Rust: Fast and prevents common bugs
- Modular Design: Easy to add new features

## Project Structure 
rust-barrier/
├── src/
│ ├── event.rs (handles mouse/keyboard events)
│ ├── network/ (handles computer communication)
│ └── lib.rs (ties everything together)
└── tests/
├── event_tests.rs (makes sure events work)
└── network_tests.rs (makes sure computers talk)

## Next Steps
- Add Linux X11 support
- Add Windows support
- Create command-line interface
- Add screen configuration

## Development Roadmap

### 1. Platform Support
- [ ] Complete Linux X11 implementation
  - [ ] Mouse event capture
  - [ ] Keyboard event capture
  - [ ] Event simulation
- [ ] Complete Windows implementation
  - [ ] Mouse event capture
  - [ ] Keyboard event capture
  - [ ] Event simulation

### 2. Command Line Interface
- [ ] Add CLI framework (clap/structopt)
- [ ] Implement configuration commands
- [ ] Add control commands
- [ ] Create help documentation

### 3. Screen Configuration
- [ ] Implement screen layout configuration
- [ ] Define relative screen positioning
- [ ] Handle screen boundaries
- [ ] Implement smooth transitions

### 4. Testing
- [ ] Complete platform-specific tests
- [ ] Add integration tests
- [ ] Implement performance benchmarks
- [ ] Add stress tests

### 5. Security
- [ ] Implement secure handshake
- [ ] Add encryption for network communication
- [ ] Complete authentication system
- [ ] Add security documentation

### 6. Performance
- [ ] Optimize event handling
- [ ] Reduce latency
- [ ] Implement proper benchmarking
- [ ] Consider alternative serialization formats
  - [ ] CBOR
  - [ ] Bincode

### 7. Documentation
- [ ] Add API documentation
- [ ] Create user setup guide
- [ ] Write configuration documentation
- [ ] Add architecture documentation

