import tkinter as tk
import random

# =========================
# 1. Definición de estados
# =========================
STATES = {
    "creacion": "lightblue",
    "corriendo": "lightgreen",
    "suspendido": "khaki",
    "detenido": "orange",
    "finalizado": "tomato"
}

# Transiciones válidas
NEXT_STATES = {
    "creacion": ["corriendo", "suspendido", "detenido"],
    "corriendo": ["suspendido", "detenido", "finalizado"],
    "suspendido": ["corriendo", "detenido", "finalizado"],
    "detenido": ["corriendo", "suspendido", "finalizado"]
}

# =========================
# 2. Generador de eventos
# =========================
class SchedulerSimulator:
    def __init__(self):
        self.pid_counter = 1
        self.process_states = {}  # PID -> estado actual

    def next_event(self):
        # 20% de probabilidad de crear un nuevo proceso
        if random.random() < 0.2:
            pid = self.pid_counter
            self.pid_counter += 1
            self.process_states[pid] = "creacion"
            return pid, "creacion"

        if not self.process_states:
            return None, None

        pid = random.choice(list(self.process_states.keys()))
        current_state = self.process_states[pid]

        if current_state == "finalizado":
            return None, None

        next_state = random.choice(NEXT_STATES[current_state])
        self.process_states[pid] = next_state
        return pid, next_state

# =========================
# 3. Interfaz gráfica
# =========================
class SchedulerGUI:
    def __init__(self, root, simulator):
        self.root = root
        self.simulator = simulator
        self.running = False
        self.delay = 1000  # ms

        self.root.title("Simulación de Scheduler de Procesos")

        # Canvas grande y expandible
        self.canvas_width = 900
        self.canvas_height = 600
        self.canvas = tk.Canvas(root, width=self.canvas_width, height=self.canvas_height, bg="white")
        self.canvas.pack(fill="both", expand=True)

        # Slider de velocidad
        self.speed_slider = tk.Scale(
            root, from_=100, to=2000, orient="horizontal",
            label="Velocidad (ms por evento)"
        )
        self.speed_slider.set(self.delay)
        self.speed_slider.pack()

        # Botones de control
        self.start_btn = tk.Button(root, text="Iniciar", command=self.start)
        self.start_btn.pack(side="left", padx=10)

        self.stop_btn = tk.Button(root, text="Detener", command=self.stop)
        self.stop_btn.pack(side="left", padx=10)

        # Posiciones de los rectángulos: 3 arriba y 2 centrados abajo
        self.state_positions = {}
        margin_x = 60
        margin_y = 80
        width = 180
        height = 200

        states_list = list(STATES.items())

        # --- fila superior (3 estados) ---
        for i, (state, color) in enumerate(states_list[:3]):
            x1 = margin_x + i * (width + margin_x)
            y1 = margin_y
            x2 = x1 + width
            y2 = y1 + height
            self._draw_state_box(state, color, x1, y1, x2, y2)

        # --- fila inferior (2 estados centrados) ---
        start_x = (self.canvas_width - (2 * width + margin_x)) // 2
        for i, (state, color) in enumerate(states_list[3:]):
            x1 = start_x + i * (width + margin_x)
            y1 = margin_y + height + margin_y
            x2 = x1 + width
            y2 = y1 + height
            self._draw_state_box(state, color, x1, y1, x2, y2)

    def _draw_state_box(self, state, color, x1, y1, x2, y2):
        self.canvas.create_rectangle(x1, y1, x2, y2, fill=color)
        self.canvas.create_text((x1 + x2) // 2, y1 - 20, text=state.upper(), font=("Arial", 12, "bold"))
        self.state_positions[state] = (x1, y1, x2, y2)

    def update_canvas(self):
        self.canvas.delete("pid")

        grouped = {state: [] for state in STATES}
        for pid, state in self.simulator.process_states.items():
            grouped[state].append(pid)

        for state, pids in grouped.items():
            x1, y1, x2, y2 = self.state_positions[state]
            for idx, pid in enumerate(pids):
                self.canvas.create_text(
                    (x1 + x2) // 2,
                    y1 + 20 + (idx * 15),
                    text=f"PID {pid}",
                    font=("Arial", 10),
                    tags="pid"
                )

    def run(self):
        if not self.running:
            return

        self.delay = self.speed_slider.get()
        pid, state = self.simulator.next_event()
        if pid is not None:
            print(f"Evento -> PID {pid}, Estado: {state}")
            self.update_canvas()

        self.root.after(self.delay, self.run)

    def start(self):
        self.running = True
        self.run()

    def stop(self):
        self.running = False

# =========================
# 4. Ejecutar simulación
# =========================
if __name__ == "__main__":
    root = tk.Tk()
    simulator = SchedulerSimulator()
    gui = SchedulerGUI(root, simulator)
    root.mainloop()
