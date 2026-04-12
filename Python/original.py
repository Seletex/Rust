import tkinter as tk
import random
from tkinter import scrolledtext, ttk

# =========================
# 1. Configuración de Estados (Estilo Original)
# =========================
STATES = {
    "creacion": "lightblue",
    "corriendo": "lightgreen",
    "suspendido": "khaki",
    "detenido": "orange",
    "finalizado": "tomato"
}

# Transiciones válidas para la simulación
NEXT_STATES = {
    "creacion": ["corriendo", "suspendido", "detenido"],
    "corriendo": ["suspendido", "detenido", "finalizado"],
    "suspendido": ["corriendo", "detenido", "finalizado"],
    "detenido": ["corriendo", "suspendido", "finalizado"]
}

# =========================
# 2. Lógica del Scheduler (Requisitos 1 y 2)
# =========================
class SchedulerSimulator:
    def __init__(self):
        self.pid_counter = 1
        self.processes = {}  # PID -> {estado, usuario, recurso}
        self.usuarios = ["usuario", "admin", "root", "estudiante"] # 4 usuarios reales
        self.recursos = [f"TCP:{p}" for p in range(80, 86)]             # TCP 80-85
        
        # Control de Recursos
        self.recurso_ocupado_por = {res: None for res in self.recursos}
        self.colas_espera = {res: [] for res in self.recursos} # Lista de espera FIFO

    def crear_proceso(self):
        pid = self.pid_counter
        self.pid_counter += 1
        self.processes[pid] = {
            "estado": "creacion",
            "usuario": random.choice(self.usuarios),
            "recurso": random.choice(self.recursos)
        }
        return pid

    def evolucionar_sistema(self):
        # 30% de probabilidad de nuevo proceso
        if random.random() < 0.3:
            self.crear_proceso()

        if not self.processes: return

        # Intentar transiciones aleatorias
        for pid in list(self.processes.keys()):
            proc = self.processes[pid]
            if proc["estado"] == "finalizado": continue
            
            if random.random() < 0.2:
                next_state = random.choice(NEXT_STATES[proc["estado"]])
                self.transicionar(pid, next_state)

    def transicionar(self, pid, destino):
        if pid not in self.processes: return
        proc = self.processes[pid]
        res = proc["recurso"]
        estado_anterior = proc["estado"]

        # Requisito 2: Lógica de paso a "corriendo"
        if destino == "corriendo":
            if self.recurso_ocupado_por[res] is None:
                # Recurso libre
                self.recurso_ocupado_por[res] = pid
                proc["estado"] = "corriendo"
                if pid in self.colas_espera[res]:
                    self.colas_espera[res].remove(pid)
            else:
                # Recurso ocupado: pasar a suspendido y cola
                if self.recurso_ocupado_por[res] != pid:
                    proc["estado"] = "suspendido"
                    if pid not in self.colas_espera[res]:
                        self.colas_espera[res].append(pid)
        else:
            # Liberar recurso si salimos de corriendo
            if estado_anterior == "corriendo":
                if self.recurso_ocupado_por[res] == pid:
                    self.recurso_ocupado_por[res] = None
                    # Activar el siguiente en la cola (FIFO)
                    if self.colas_espera[res]:
                        sig_pid = self.colas_espera[res][0]
                        self.transicionar(sig_pid, "corriendo")
            
            proc["estado"] = destino

# =========================
# 3. Interfaz Gráfica (Estilo Requerido)
# =========================
class SchedulerGUI:
    def __init__(self, root, simulator):
        self.root = root
        self.simulator = simulator
        self.running = False
        self.delay = 1000

        self.root.title("Simulación de Scheduler - Requisitos Completos")
        self.root.geometry("1100x900")

        # --- 1. Botones de Control (Ahora en la parte superior para visibilidad) ---
        self.header = tk.Frame(root, bg="#f0f0f0", pady=10)
        self.header.pack(fill="x")
        
        tk.Label(self.header, text="CONTROL DE SIMULACIÓN:", font=("Arial", 10, "bold"), bg="#f0f0f0").pack(side="left", padx=20)
        
        self.start_btn = tk.Button(self.header, text="▶ INICIAR", command=self.start, bg="#27AE60", fg="white", font=("Arial", 9, "bold"), padx=20)
        self.start_btn.pack(side="left", padx=10)
        
        self.stop_btn = tk.Button(self.header, text="⏹ DETENER", command=self.stop, bg="#C0392B", fg="white", font=("Arial", 9, "bold"), padx=20)
        self.stop_btn.pack(side="left", padx=10)

        # --- 2. Monitor Visual de Recursos (Requisito 3 - Muy visible) ---
        self.res_monitor_frame = tk.Frame(root, bg="white", pady=10)
        self.res_monitor_frame.pack(fill="x", padx=10)
        
        tk.Label(self.res_monitor_frame, text="ESTADO DE PUERTOS TCP (VERDE=LIBRE, ROJO=OCUPADO):", font=("Arial", 9, "bold"), bg="white").pack()
        
        self.res_canvas = tk.Canvas(self.res_monitor_frame, height=80, bg="white", highlightthickness=0)
        self.res_canvas.pack(fill="x")

        # --- 3. Canvas para estados (Bloques originales) ---
        self.canvas_width = 800
        self.canvas_height = 420
        self.canvas = tk.Canvas(root, width=self.canvas_width, height=self.canvas_height, bg="white")
        self.canvas.pack(fill="x", padx=10)

        # --- 4. Terminal (Requisito 4) ---
        tk.Label(root, text="OPERACIONES DE SISTEMA (Consola):", font=("Arial", 9, "bold")).pack(anchor="w", padx=20)
        
        # Frame principal de la terminal ocupando el resto del espacio
        self.terminal_frame = tk.Frame(root, bg="black", borderwidth=2, relief="sunken")
        self.terminal_frame.pack(fill="both", expand=True, padx=20, pady=(0, 10))
        
        # Primero pack el área de entrada abajo para asegurar que sea visible
        self.input_line = tk.Frame(self.terminal_frame, bg="black", pady=5)
        self.input_line.pack(side="bottom", fill="x", padx=5)
        
        self.prompt_label = tk.Label(self.input_line, text="root@simuos:~$ ", bg="black", fg="#00FF00", font=("Consolas", 10, "bold"))
        self.prompt_label.pack(side="left")

        self.cmd_input = tk.Entry(self.input_line, bg="#111111", fg="white", font=("Consolas", 10), 
                                 insertbackground="white", borderwidth=1, relief="flat")
        self.cmd_input.pack(side="left", fill="x", expand=True)
        self.cmd_input.bind("<Return>", self.ejecutar_comando)
        
        # Luego el área de texto arriba ocupando todo el espacio restante
        self.txt_terminal = scrolledtext.ScrolledText(self.terminal_frame, bg="black", fg="#2ECC71", 
                                                     font=("Consolas", 10), borderwidth=0, height=5)
        self.txt_terminal.pack(side="top", fill="both", expand=True)
        self.txt_terminal.configure(state='disabled')
        
        # Forzar foco
        self.cmd_input.focus_set()

        # Posiciones de los bloques de estado (3 arriba, 2 abajo)
        self.state_positions = {}
        self.setup_state_boxes()
        self.update_resources_visual()

    def setup_state_boxes(self):
        margin_x = 40
        margin_y = 50
        width = 200
        height = 180
        states_list = list(STATES.items())

        # Fila superior (3)
        for i, (state, color) in enumerate(states_list[:3]):
            x1 = margin_x + i * (width + margin_x)
            y1 = margin_y
            x2 = x1 + width
            y2 = y1 + height
            self._draw_state_box(state, color, x1, y1, x2, y2)

        # Fila inferior (2)
        start_x = (self.canvas_width - (2 * width + margin_x)) // 2
        for i, (state, color) in enumerate(states_list[3:]):
            x1 = start_x + i * (width + margin_x)
            y1 = margin_y + height + margin_y
            x2 = x1 + width
            y2 = y1 + height
            self._draw_state_box(state, color, x1, y1, x2, y2)

    def _draw_state_box(self, state, color, x1, y1, x2, y2):
        self.canvas.create_rectangle(x1, y1, x2, y2, fill=color, outline="black", tags=f"box_{state}")
        self.canvas.create_text((x1 + x2) // 2, y1 - 15, text=state.upper(), font=("Arial", 10, "bold"))
        self.state_positions[state] = (x1, y1, x2, y2)

    def update_resources_visual(self):
        self.res_canvas.delete("all")
        
        n_res = len(self.simulator.recursos)
        canvas_w = 1000 # Estimado
        block_w = 140
        spacing = 20
        total_w = (block_w * n_res) + (spacing * (n_res - 1))
        x_offset = (1100 - total_w) // 2

        for i, res in enumerate(self.simulator.recursos):
            owner = self.simulator.recurso_ocupado_por[res]
            queue = self.simulator.colas_espera[res]
            color = "#E74C3C" if owner else "#2ECC71" # Rojo si ocupado, Verde si libre
            
            x1 = x_offset + i * (block_w + spacing)
            y1 = 10
            x2 = x1 + block_w
            y2 = y1 + 50
            
            # Dibujar el bloque de color (Req 3)
            self.res_canvas.create_rectangle(x1, y1, x2, y2, fill=color, outline="black", width=2)
            self.res_canvas.create_text((x1+x2)//2, y1+15, text=res, font=("Arial", 10, "bold"), fill="white" if owner else "black")
            
            # Texto de dueño y cola
            owner_txt = f"PID: {owner}" if owner else "LIBRE"
            self.res_canvas.create_text((x1+x2)//2, y1+35, text=owner_txt, font=("Arial", 8), fill="white" if owner else "black")
            
            if queue:
                queue_txt = f"Cola: {queue[:2]}"
                self.res_canvas.create_text((x1+x2)//2, y2+10, text=queue_txt, font=("Arial", 7), fill="black")

    def write_terminal(self, text):
        self.txt_terminal.configure(state='normal')
        self.txt_terminal.insert(tk.END, f"{text}\n")
        self.txt_terminal.see(tk.END)
        self.txt_terminal.configure(state='disabled')

    def ejecutar_comando(self, event):
        cmd_text = self.cmd_input.get().strip()
        self.cmd_input.delete(0, tk.END)
        if not cmd_text: return

        self.write_terminal(f"> {cmd_text}")
        parts = cmd_text.split()
        cmd = parts[0].lower()

        if cmd == "ps":
            username = None
            if len(parts) >= 3 and parts[1].lower() == "-u":
                username = parts[2].lower()
            
            self.write_terminal(f"{'PID':<6} | {'OWNER':<12} | {'STATE':<12}")
            self.write_terminal("-" * 35)
            for pid, info in self.simulator.processes.items():
                # Filtro por usuario (ahora insensible a mayúsculas/minúsculas)
                if username and info["usuario"].lower() != username: continue
                if info["estado"] == "finalizado": continue
                self.write_terminal(f"{pid:<6} | {info['usuario']:<12} | {info['estado']:<12}")
        
        elif cmd == "kill" and len(parts) > 1:
            try:
                pid = int(parts[1])
                if pid in self.simulator.processes:
                    self.simulator.transicionar(pid, "finalizado")
                    self.write_terminal(f"Proceso {pid} finalizado.")
                else:
                    self.write_terminal(f"Error: PID {pid} no encontrado.")
            except ValueError:
                self.write_terminal("Error: PID inválido.")
        
        elif cmd == "stress":
            self.write_terminal("Inyectando 20 procesos al sistema...")
            for _ in range(20):
                self.simulator.crear_proceso()
            self.write_terminal("Carga de estrés completada.")

        elif cmd == "help":
            self.write_terminal("Comandos: ps, ps -u [user], kill [pid], stress, help")
        
        self.update_canvas()
        self.update_resources_visual()

    def update_canvas(self):
        self.canvas.delete("pid_text")
        grouped = {state: [] for state in STATES}
        for pid, info in self.simulator.processes.items():
            # Mostramos todos los estados, incluyendo FINALIZADO
            grouped[info["estado"]].append((pid, info["usuario"], info["recurso"]))

        for state, items in grouped.items():
            x1, y1, x2, y2 = self.state_positions[state]
            for idx, (pid, user, res) in enumerate(items):
                if y1 + 20 + (idx * 15) > y2 - 10: break # Limite de caja
                self.canvas.create_text(
                    (x1 + x2) // 2,
                    y1 + 20 + (idx * 15),
                    text=f"PID {pid} ({user}) [{res}]",
                    font=("Arial", 8),
                    tags="pid_text"
                )

    def run(self):
        if not self.running: return
        self.simulator.evolucionar_sistema()
        self.update_canvas()
        self.update_resources_visual()
        self.root.after(self.delay, self.run)

    def start(self):
        if not self.running:
            self.running = True
            self.run()

    def stop(self):
        self.running = False

# =========================
# 4. Main
# =========================
if __name__ == "__main__":
    root = tk.Tk()
    sim = SchedulerSimulator()
    gui = SchedulerGUI(root, sim)
    root.mainloop()
