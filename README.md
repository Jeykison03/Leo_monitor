

Real-time heart rate monitoring system with a Rust backend and Vue 3 frontend.



**Important:** Hardware is not available in the cloud. You **must enable simulation mode**

1.  **Backend** (Terminal 1):
    ```bash
    cd backend
    SIMULATION_MODE=true cargo run
    ```

2.  **Frontend** (Terminal 2):
    ```bash
    cd frontend
    npm run dev
    ```

## Features
- **Real-time Visualization**: Live heart rate graph.
- **Roles**: Doctor & Assistant login support.
- **Mock Data**: Simulation mode for cloud development.

## Project Structure
- `backend/`: Rust (Actix-web, MongoDB).
- `frontend/`: Vue 3, Vite, D3.js.
- `arduino/`: Hardware C++ code.
