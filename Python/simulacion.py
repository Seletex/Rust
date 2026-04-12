import tkinter as tk
import random
from tkinter import messagebox, scrolledtext, ttk

# =========================
# 1. Configuración de Estados y Colores
# =========================
STATES = {
    "creacion": "#D1E8FF",    # Celeste
    "corriendo": "#B9F6BC",   # Verde Lima
    "suspendido": "#FFF59D",  # Amarillo (Esperando recurso)
    "detenido": "#FFCC80",    # Naranja
    "finalizado": "#EF9A9A"   # Rojo suave
}

# Transiciones permitidas
POSSIBLE_NEXT = {
    "creacion": ["corriendo", "detenido"],
    "corriendo": ["detenido", "suspendido", "finalizado"],
    "suspendido": ["corriendo", "detenido"],
    "detenido": ["corriendo", "suspendido", "finalizado"]
}

class SchedulerSimulator:
    def __init__(self):
        self.pid_counter = 1
        self.processes = {}  # Tabla de procesos: {pid: {estado, usuario, recurso}}
        self.usuarios = ["root", "admin", "estudiante", "invitado"]
        self.recursos = [f"TCP:{p}" for p in range(80, 86)] # Puertos 80-85
        
        # Control de Exclusión Mutua
        self.recurso_ocupado_por = {res: None for res in self.recursos}
        self.colas_espera = {res: [] for res in self.recursos}

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
        # 1. Probabilidad de nacimiento de procesos (Flujo más agresivo)
        if random.random() < 0.5:
            for _ in range(random.randint(2, 5)):
                self.crear_proceso()

        # 2. Intentar transiciones aleatorias
        for pid in list(self.processes.keys()):
            proc = self.processes[pid]
            if proc["estado"] == "finalizado": continue

            if random.random() < 0.4:
                # REGLA DE LONGEVIDAD: Menos probabilidad de finalizar rápido
                opciones = POSSIBLE_NEXT[proc["estado"]]
                if "finalizado" in opciones and random.random() < 0.7:
                     # Evitamos 'finalizado' el 70% de las veces para que vivan más
                     opciones_sin_fin = [o for o in opciones if o != "finalizado"]
                     nuevo_estado = random.choice(opciones_sin_fin if opciones_sin_fin else opciones)
                else:
                    nuevo_estado = random.choice(opciones)
                
                self.transicionar(pid, nuevo_estado)

    def transicionar(self, pid, destino):
        if pid not in self.processes: return
        proc = self.processes[pid]
        res = proc["recurso"]
        estado_anterior = proc["estado"]

        # Lógica de Competencia por Recurso (Req 2)
        if destino == "corriendo":
            # Si el recurso está libre, lo toma
            if self.recurso_ocupado_por[res] is None:
                self.recurso_ocupado_por[res] = pid
                proc["estado"] = "corriendo"
                # Si estaba en cola, lo quitamos
                if pid in self.colas_espera[res]:
                    self.colas_espera[res].remove(pid)
            else:
                # Si está ocupado por otro PID, va a suspendido y cola
                if self.recurso_ocupado_por[res] != pid:
                    proc["estado"] = "suspendido"
                    if pid not in self.colas_espera[res]:
                        self.colas_espera[res].append(pid)
        else:
            # Si salimos de "corriendo", liberamos el recurso
            if estado_anterior == "corriendo":
                self.recurso_ocupado_por[res] = None
                # Activar al siguiente en la lista de espera (orden FIFO)
                if self.colas_espera[res]:
                    siguiente_pid = self.colas_espera[res][0] # No hacemos pop aquí para que transicionar lo maneje
                    self.transicionar(siguiente_pid, "corriendo")
            
            proc["estado"] = destino

class SchedulerGUI:
    def __init__(self, root, sim):
        self.root = root
        self.root.title("OS Scheduler Simulator - Professional Edition")
        self.root.geometry("1150x800")
        self.root.configure(bg="#2C3E50")
        self.sim = sim
        self.running = False
        
        # --- Layout Principal ---
        self.top_frame = tk.Frame(root, bg="#2C3E50")
        self.top_frame.pack(fill="both", expand=True, padx=10, pady=5)
        
        self.canvas = tk.Canvas(self.top_frame, width=1100, height=420, bg="#FDFEFE", highlightthickness=0)
        self.canvas.pack(pady=10)

        # Botones de Control
        self.ctrl_frame = tk.Frame(self.top_frame, bg="#2C3E50")
        self.ctrl_frame.pack(fill="x")

        self.btn_start = tk.Button(self.ctrl_frame, text="▶ INICIAR SIMULACIÓN", command=self.toggle, bg="#27AE60", fg="white", font=("Arial", 10, "bold"), relief="flat", padx=20)
        self.btn_start.pack(side="left", padx=10)

        self.btn_reporte = tk.Button(self.ctrl_frame, text="📊 VER HISTORIAL COMPLETO", command=self.abrir_reporte, bg="#2980B9", fg="white", font=("Arial", 10, "bold"), relief="flat", padx=20)
        self.btn_reporte.pack(side="left", padx=10)

        # --- Terminal Simulator (Req 4) ---
        self.bottom_frame = tk.Frame(root, bg="#1C2833")
        self.bottom_frame.pack(fill="x", side="bottom", padx=10, pady=10)
        
        tk.Label(self.bottom_frame, text="SYSTEM TERMINAL", bg="#1C2833", fg="#ABB2B9", font=("Courier", 10, "bold")).pack(anchor="w", padx=5)
        
        self.txt_terminal = scrolledtext.ScrolledText(self.bottom_frame, height=10, bg="black", fg="#2ECC71", font=("Consolas", 10))
        self.txt_terminal.pack(fill="x", padx=5, pady=2)
        self.txt_terminal.insert(tk.END, "Welcome to SimuOS v2.5\nType 'help' for commands.\n\n")
        self.txt_terminal.configure(state='disabled')

        self.cmd_input_frame = tk.Frame(self.bottom_frame, bg="#1C2833")
        self.cmd_input_frame.pack(fill="x", padx=5, pady=5)
        
        tk.Label(self.cmd_input_frame, text="user@simuos:~# ", bg="#1C2833", fg="#F1C40F", font=("Consolas", 10, "bold")).pack(side="left")
        
        self.cmd_entry = tk.Entry(self.cmd_input_frame, bg="black", fg="white", font=("Consolas", 10), insertbackground="white", borderwidth=0)
        self.cmd_entry.pack(side="left", fill="x", expand=True)
        self.cmd_entry.bind("<Return>", self.ejecutar_comando)

        # Botón de Inicio
        self.btn_start = tk.Button(self.top_frame, text="START SIMULATION", command=self.toggle, bg="#27AE60", fg="white", font=("Arial", 10, "bold"), relief="flat", padx=20)
        self.btn_start.pack(pady=5)

        self.update_view()

    def write_terminal(self, text):
        self.txt_terminal.configure(state='normal')
        self.txt_terminal.insert(tk.END, text + "\n")
        self.txt_terminal.see(tk.END)
        self.txt_terminal.configure(state='disabled')

    def toggle(self):
        self.running = not self.running
        self.btn_start.config(text="⏸ PAUSAR" if self.running else "▶ CONTINUAR", bg="#E67E22" if self.running else "#27AE60")
        if self.running: self.loop()

    def loop(self):
        if self.running:
            self.sim.evolucionar_sistema()
            self.update_view()
            self.root.after(450, self.loop)

    def ejecutar_comando(self, event):
        raw_text = self.cmd_entry.get().strip()
        if not raw_text: return
        
        self.write_terminal(f"user@simuos:~# {raw_text}")
        parts = raw_text.split()
        cmd = parts[0].lower()

        # Lógica de Comandos (Req 4 + Mejoras)
        if cmd == "ps":
            if len(parts) >= 3 and parts[1] == "-u":
                self.cmd_ps(username=parts[2])
            elif len(parts) >= 2 and parts[1] == "-f":
                self.cmd_ps(only_finished=True)
            elif len(parts) >= 2 and parts[1] == "-a":
                self.cmd_ps(show_all=True)
            else:
                self.cmd_ps()
        
        elif cmd == "stress":
            self.write_terminal("Inyectando 20 procesos al sistema...")
            for _ in range(20):
                self.sim.crear_proceso()
        
        elif cmd == "report":
            self.abrir_reporte()

        elif cmd == "kill" and len(parts) > 1:
            try:
                pid = int(parts[1])
                if pid in self.sim.processes:
                    self.sim.transicionar(pid, "finalizado")
                    self.write_terminal(f"Process {pid} terminated.")
                else:
                    self.write_terminal(f"Error: PID {pid} not found.")
            except ValueError:
                self.write_terminal("Error: Invalid PID.")
        
        elif cmd == "help":
            self.write_terminal("Commands: ps, ps -u [user], ps -f (fin), ps -a (all), stress, report, kill [pid], clear")
        
        elif cmd == "clear":
            self.txt_terminal.configure(state='normal')
            self.txt_terminal.delete('1.0', tk.END)
            self.txt_terminal.configure(state='disabled')
        
        else:
            self.write_terminal(f"Command not found: {cmd}")

        self.cmd_entry.delete(0, tk.END)
        self.update_view()

    def cmd_ps(self, username=None, only_finished=False, show_all=False):
        header = f"{'PID':<6} | {'OWNER':<12} | {'STATE':<12} | {'RESOURCE':<8}"
        self.write_terminal(header)
        self.write_terminal("-" * 50)
        
        if username:
            username = username.strip("[]").lower()

        found = False
        for pid, info in self.sim.processes.items():
            # Filtro por estado
            if only_finished and info["estado"] != "finalizado": continue
            if not show_all and not only_finished and info["estado"] == "finalizado": continue
            
            # Filtro por usuario
            if username and info["usuario"].lower() != username:
                continue
                
            line = f"{pid:<6} | {info['usuario']:<12} | {info['estado']:<12} | {info['recurso']:<8}"
            self.write_terminal(line)
            found = True
            
        if not found:
            self.write_terminal("No processes found for this filter.")

    def abrir_reporte(self):
        # Crear una ventana de reporte independiente
        v_repo = tk.Toplevel(self.root)
        v_repo.title("Reporte Detallado de Procesos")
        v_repo.geometry("700x500")
        
        # Frame para la tabla
        frame = tk.Frame(v_repo)
        frame.pack(fill="both", expand=True, padx=10, pady=10)
        
        # Barra de desplazamiento
        scrollbar = tk.Scrollbar(frame)
        scrollbar.pack(side="right", fill="y")
        
        # Configuración de Treeview (Tabla)
        columnas = ("PID", "Usuario", "Estado Final", "Puerto TCP")
        tabla = ttk.Treeview(frame, columns=columnas, show="headings", yscrollcommand=scrollbar.set)
        tabla.pack(fill="both", expand=True)
        scrollbar.config(command=tabla.yview)
        
        # Definir cabeceras
        for col in columnas:
            tabla.heading(col, text=col)
            tabla.column(col, anchor="center", width=120)

        # Llenar con todos los procesos
        for pid, info in self.sim.processes.items():
            tabla.insert("", tk.END, values=(pid, info["usuario"], info["estado"], info["recurso"]))

        tk.Label(v_repo, text=f"Total de registros: {len(self.sim.processes)}", font=("Arial", 9, "italic")).pack(pady=5)

    def update_view(self):
        self.canvas.delete("all")
        
        # --- Estadísticas Generales ---
        total = len(self.sim.processes)
        finished = len([p for p in self.sim.processes.values() if p["estado"] == "finalizado"])
        active = total - finished
        stats = f"PROCESOS ACTIVOS: {active} | FINALIZADOS: {finished} | TOTAL: {total}"
        self.canvas.create_text(20, 20, text=stats, font=("Arial", 9, "bold"), fill="#1F618D", anchor="w")

        # --- Dibujar Recursos TCP (Req 3) ---
        self.canvas.create_text(950, 30, text="RESOURCE STATUS (TCP)", font=("Arial", 10, "bold"), fill="#2C3E50")
        for i, res in enumerate(self.sim.recursos):
            owner = self.sim.recurso_ocupado_por[res]
            wait_list = self.sim.colas_espera[res]
            
            y_base = 60 + (i * 55)
            color = "#E74C3C" if owner else "#2ECC71"
            
            self.canvas.create_rectangle(850, y_base, 880, y_base+30, fill=color, outline="#34495E")
            self.canvas.create_text(900, y_base+15, text=f"{res}", font=("Consolas", 9, "bold"), anchor="w")
            
            status_text = f"Owner: {f'PID {owner}' if owner else 'FREE'}"
            wait_display = str(wait_list[:4]) + ("..." if len(wait_list) > 4 else "")
            wait_text = f"Wait List: {wait_display if wait_list else 'None'}"
            
            self.canvas.create_text(980, y_base+5, text=status_text, font=("Consolas", 8), anchor="w")
            self.canvas.create_text(980, y_base+20, text=wait_text, font=("Consolas", 8), anchor="w", fill="#7F8C8D")

        # --- Dibujar Columnas de Estado ---
        cols_to_show = ["creacion", "corriendo", "suspendido", "detenido"]
        for i, state in enumerate(cols_to_show):
            x = 20 + (i * 200)
            self.canvas.create_rectangle(x, 50, x+185, 400, outline="#D5DBDB", fill="#F8F9F9", width=1)
            self.canvas.create_text(x+92, 35, text=state.upper(), font=("Arial", 9, "bold"), fill="#34495E")
            
            proc_y = 70
            for pid, info in self.sim.processes.items():
                if info["estado"] == state:
                    self.canvas.create_rectangle(x+10, proc_y, x+175, proc_y+40, fill=STATES[state], outline="#ABB2B9")
                    text_info = f"PID: {pid} | {info['usuario']}\n{info['recurso']}"
                    self.canvas.create_text(x+92, proc_y+20, text=text_info, font=("Arial", 7), justify="center")
                    proc_y += 50
                    if proc_y > 360: break

if __name__ == "__main__":
    root = tk.Tk()
    sim = SchedulerSimulator()
    app = SchedulerGUI(root, sim)
    root.mainloop()