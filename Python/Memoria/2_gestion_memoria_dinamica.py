import tkinter as tk
from tkinter import ttk, messagebox
from dataclasses import dataclass, field
import random
import time

# -------------------------
# Modelo de memoria física
# -------------------------

@dataclass
class Segment:
    start_kb: int
    size_kb: int
    kind: str           # 'kernel', 'proc', 'dyn'
    pid: int | None = None
    label: str = ""
    color: str = "#999999"
    rel_start_kb: int | None = None  # NUEVO: dirección relativa (virtual) para el proceso

    @property
    def end_kb(self):
        return self.start_kb + self.size_kb

    @property
    def rel_end_kb(self):
        if self.rel_start_kb is None:
            return None
        return self.rel_start_kb + self.size_kb


@dataclass
class MapEntry:
    rel_start_kb: int
    size_kb: int
    phys_start_kb: int
    kind: str           # 'proc' o 'dyn'
    label: str

    @property
    def rel_end_kb(self):
        return self.rel_start_kb + self.size_kb

    @property
    def phys_end_kb(self):
        return self.phys_start_kb + self.size_kb


@dataclass
class Process:
    pid: int
    name: str
    color: str
    segments: list[Segment] = field(default_factory=list)

    # NUEVO: administración de direcciones relativas (monótona, tipo heap creciente)
    next_rel_kb: int = 0
    mem_map: list[MapEntry] = field(default_factory=list)

    @property
    def mem_kb(self):
        return sum(s.size_kb for s in self.segments)

    def allocate_rel(self, size_kb: int) -> int:
        """Devuelve el inicio relativo y avanza el puntero (NO reutiliza huecos)."""
        base = self.next_rel_kb
        self.next_rel_kb += size_kb
        return base

    def add_mapping(self, seg: Segment):
        if seg.rel_start_kb is None:
            return
        self.mem_map.append(
            MapEntry(
                rel_start_kb=seg.rel_start_kb,
                size_kb=seg.size_kb,
                phys_start_kb=seg.start_kb,
                kind=seg.kind,
                label=seg.label
            )
        )
        # opcional: ordenar por dirección relativa para visualizar
        self.mem_map.sort(key=lambda e: e.rel_start_kb)

    def remove_mapping_by_rel_start(self, rel_start_kb: int):
        self.mem_map = [m for m in self.mem_map if m.rel_start_kb != rel_start_kb]


class MemoryModel:
    def __init__(self, total_kb: int, kernel_kb: int):
        if kernel_kb >= total_kb:
            raise ValueError("El kernel no puede ser >= RAM total.")
        self.total_kb = total_kb
        self.kernel_kb = kernel_kb
        self.segments: list[Segment] = []
        self.segments.append(
            Segment(0, kernel_kb, kind="kernel", pid=None, label="kernel", color="#444444")
        )

    def allocated_segments_sorted(self):
        return sorted(self.segments, key=lambda s: s.start_kb)

    def used_kb(self):
        return sum(s.size_kb for s in self.segments)

    def free_kb(self):
        return self.total_kb - self.used_kb()

    def free_segments(self):
        """Huecos libres [a,b) en KB."""
        segs = self.allocated_segments_sorted()
        gaps = []
        cursor = 0
        for s in segs:
            if s.start_kb > cursor:
                gaps.append((cursor, s.start_kb))
            cursor = max(cursor, s.end_kb)
        if cursor < self.total_kb:
            gaps.append((cursor, self.total_kb))

        kernel_end = self.kernel_kb
        cleaned = []
        for a, b in gaps:
            if b <= kernel_end:
                continue
            cleaned.append((max(a, kernel_end), b))
        return [(a, b) for a, b in cleaned if b > a]

    def can_allocate_at(self, start_kb: int, size_kb: int):
        if start_kb < self.kernel_kb:
            return False
        end_kb = start_kb + size_kb
        if end_kb > self.total_kb:
            return False
        for a, b in self.free_segments():
            if start_kb >= a and end_kb <= b:
                return True
        return False

    def allocate(self, start_kb: int, size_kb: int, kind: str, pid: int,
                 label: str, color: str, rel_start_kb: int | None = None):
        if not self.can_allocate_at(start_kb, size_kb):
            raise ValueError("No hay hueco contiguo suficiente en esa posición.")
        seg = Segment(
            start_kb=start_kb, size_kb=size_kb, kind=kind, pid=pid,
            label=label, color=color, rel_start_kb=rel_start_kb
        )
        self.segments.append(seg)
        return seg

    def free_by_pid(self, pid: int):
        self.segments = [s for s in self.segments if s.kind == "kernel" or s.pid != pid]

    def free_segment(self, segment: Segment):
        if segment.kind == "kernel":
            return
        self.segments = [s for s in self.segments if s is not segment]


# -------------------------
# Simulación / Gamificación
# -------------------------

APP_NAMES = [
    "bash", "sshd", "python3", "firefox", "vim", "nano", "nginx",
    "postgres", "docker", "systemd", "pulseaudio", "code", "java",
    "chrome", "gcc", "make", "node", "redis", "cupsd", "top"
]

PALETTE = [
    "#60a5fa", "#34d399", "#fbbf24", "#f87171", "#a78bfa",
    "#fb7185", "#22c55e", "#14b8a6", "#f97316", "#e879f9"
]

@dataclass
class PendingRequest:
    kind: str           # 'launch' or 'alloc'
    pid: int
    proc_name: str
    size_kb: int
    created_at: float
    time_limit: float
    label: str


class SimulatorState:
    def __init__(self):
        self.pid_counter = 1000
        self.processes: dict[int, Process] = {}
        self.score = 0
        self.lives = 5
        self.pending_queue: list[PendingRequest] = []
        self.current_pending: PendingRequest | None = None
        self.running = True


# -------------------------
# Interfaz Gráfica (Tkinter)
# -------------------------

class MemorySimApp(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("Simulador de Gestión de Memoria (SO) — Gamificado")
        self.geometry("1200x720")
        self.minsize(1100, 650)

        self.model: MemoryModel | None = None
        self.state = SimulatorState()

        self.mem_x0 = 40
        self.mem_y0 = 40
        self.mem_w = 280
        self.mem_h = 560

        self.px_per_kb = 0.5
        self.pending_block_id = None
        self.pending_text_id = None
        self.drag_offset = (0, 0)

        self.default_time_limit = 12.0

        self._build_config_screen()

    # --------------- Screens ---------------

    def _build_config_screen(self):
        self.config_frame = ttk.Frame(self, padding=20)
        self.config_frame.pack(fill="both", expand=True)

        title = ttk.Label(self.config_frame, text="Configuración Inicial", font=("Segoe UI", 18, "bold"))
        title.pack(anchor="w", pady=(0, 10))

        desc = ttk.Label(
            self.config_frame,
            text=("Define la RAM total (KB) y la memoria reservada por el kernel (KB).\n"
                  "Luego podrás simular procesos, asignar bloques arrastrándolos y ganar puntos por velocidad."),
            font=("Segoe UI", 10)
        )
        desc.pack(anchor="w", pady=(0, 18))

        form = ttk.Frame(self.config_frame)
        form.pack(anchor="w", pady=10)

        ttk.Label(form, text="RAM total (KB):").grid(row=0, column=0, sticky="w", padx=(0, 10), pady=6)
        self.total_entry = ttk.Entry(form, width=20)
        self.total_entry.insert(0, "65536")
        self.total_entry.grid(row=0, column=1, sticky="w")

        ttk.Label(form, text="RAM kernel (KB):").grid(row=1, column=0, sticky="w", padx=(0, 10), pady=6)
        self.kernel_entry = ttk.Entry(form, width=20)
        self.kernel_entry.insert(0, "8192")
        self.kernel_entry.grid(row=1, column=1, sticky="w")

        ttk.Label(form, text="Vidas (intentos):").grid(row=2, column=0, sticky="w", padx=(0, 10), pady=6)
        self.lives_entry = ttk.Entry(form, width=20)
        self.lives_entry.insert(0, "5")
        self.lives_entry.grid(row=2, column=1, sticky="w")

        ttk.Label(form, text="Tiempo por asignación (s):").grid(row=3, column=0, sticky="w", padx=(0, 10), pady=6)
        self.time_entry = ttk.Entry(form, width=20)
        self.time_entry.insert(0, "12")
        self.time_entry.grid(row=3, column=1, sticky="w")

        btn = ttk.Button(self.config_frame, text="Iniciar simulador", command=self._start_simulation)
        btn.pack(anchor="w", pady=20)

        tips = ttk.Label(
            self.config_frame,
            text=("Tip: el bloque aparece fuera de la RAM. Arrástralo dentro del rectángulo de memoria.\n"
                  "Ahora cada proceso maneja un mapa: direcciones relativas consecutivas → direcciones físicas (fragmentadas)."),
            font=("Segoe UI", 9, "italic")
        )
        tips.pack(anchor="w", pady=(10, 0))

    def _start_simulation(self):
        try:
            total_kb = int(self.total_entry.get().strip())
            kernel_kb = int(self.kernel_entry.get().strip())
            self.state.lives = int(self.lives_entry.get().strip())
            self.default_time_limit = float(self.time_entry.get().strip())
            if total_kb <= 0 or kernel_kb <= 0:
                raise ValueError
            self.model = MemoryModel(total_kb, kernel_kb)
        except Exception:
            messagebox.showerror("Error", "Verifica que los valores sean enteros positivos y kernel < total.")
            return

        self.px_per_kb = self.mem_h / self.model.total_kb

        self.config_frame.destroy()
        self._build_main_ui()
        self._log_event("systemd", "INFO", f"Simulación iniciada: RAM={total_kb}KB, kernel={kernel_kb}KB")

        self.after(80, self._tick_game)
        self.after(1200, self._tick_simulation_events)

    def _build_main_ui(self):
        root = ttk.Frame(self, padding=10)
        root.pack(fill="both", expand=True)

        # Left: memory canvas
        left = ttk.Frame(root)
        left.pack(side="left", fill="both", expand=False, padx=(0, 12))

        status = ttk.Frame(left)
        status.pack(fill="x", pady=(0, 8))

        self.lbl_stats = ttk.Label(status, text="", font=("Segoe UI", 10, "bold"))
        self.lbl_stats.pack(side="left")

        self.lbl_score = ttk.Label(status, text="", font=("Segoe UI", 10, "bold"))
        self.lbl_score.pack(side="right")

        self.canvas = tk.Canvas(left, width=520, height=640, bg="#0b1220", highlightthickness=0)
        self.canvas.pack()

        self._redraw_memory()

        self.lbl_pending = ttk.Label(left, text="Evento actual: (ninguno)", font=("Segoe UI", 10))
        self.lbl_pending.pack(fill="x", pady=(8, 0))

        self.pb_time = ttk.Progressbar(left, orient="horizontal", length=520, mode="determinate")
        self.pb_time.pack(fill="x", pady=(6, 0))

        # Right side
        right = ttk.Frame(root)
        right.pack(side="left", fill="both", expand=True)

        controls = ttk.Frame(right)
        controls.pack(fill="x")

        ttk.Button(controls, text="Simular lanzamiento", command=self._simulate_launch).pack(side="left", padx=(0, 8))
        ttk.Button(controls, text="Simular terminación", command=self._simulate_termination).pack(side="left", padx=(0, 8))

        # NUEVO botón: ver mapa de memoria
        self.btn_map = ttk.Button(controls, text="Ver mapa de memoria", command=self._open_memory_map, state="disabled")
        self.btn_map.pack(side="left", padx=(0, 8))

        ttk.Button(controls, text="Pausar/Reanudar", command=self._toggle_pause).pack(side="left", padx=(0, 8))
        ttk.Button(controls, text="Reiniciar", command=self._restart).pack(side="right")

        table_frame = ttk.LabelFrame(right, text="Tabla de procesos")
        table_frame.pack(fill="x", pady=10)

        cols = ("PID", "Proceso", "Memoria (KB)", "% RAM")
        self.tree = ttk.Treeview(table_frame, columns=cols, show="headings", height=8)
        for c in cols:
            self.tree.heading(c, text=c)
            self.tree.column(c, width=110 if c != "Proceso" else 150, anchor="center")
        self.tree.pack(fill="x", padx=8, pady=8)

        # Selección habilita botón mapa
        self.tree.bind("<<TreeviewSelect>>", self._on_process_select)

        console_frame = ttk.LabelFrame(right, text="Consola de eventos")
        console_frame.pack(fill="both", expand=True, pady=(0, 0))

        self.console = tk.Text(console_frame, height=18, bg="#050a14", fg="#d1d5db",
                               insertbackground="#d1d5db", wrap="word")
        self.console.pack(fill="both", expand=True, padx=8, pady=8)
        self.console.tag_config("INFO", foreground="#93c5fd")
        self.console.tag_config("WARN", foreground="#fbbf24")
        self.console.tag_config("ERROR", foreground="#f87171")
        self.console.tag_config("OK", foreground="#34d399")

        self._update_stats()

    # --------------- Drawing & UI Updates ---------------

    def _kb_to_y(self, kb: int):
        return self.mem_y0 + kb * self.px_per_kb

    def _y_to_kb(self, y: float):
        kb = int(round((y - self.mem_y0) / self.px_per_kb))
        return max(0, min(kb, self.model.total_kb))

    def _redraw_memory(self):
        self.canvas.delete("all")

        x0, y0 = self.mem_x0, self.mem_y0
        x1, y1 = x0 + self.mem_w, y0 + self.mem_h
        self.canvas.create_rectangle(x0, y0, x1, y1, outline="#94a3b8", width=2)
        self.canvas.create_text((x0 + x1) / 2, y0 - 16, text="Memoria RAM (KB)",
                                fill="#e5e7eb", font=("Segoe UI", 11, "bold"))

        k_end = self._kb_to_y(self.model.kernel_kb)
        self.canvas.create_rectangle(x0, y0, x1, k_end, fill="#1f2937", outline="")
        self.canvas.create_text(x0 + 8, y0 + 10, anchor="nw",
                                text=f"kernel: {self.model.kernel_kb}KB",
                                fill="#e5e7eb", font=("Segoe UI", 9, "bold"))

        for a, b in self.model.free_segments():
            ya = self._kb_to_y(a)
            yb = self._kb_to_y(b)
            self.canvas.create_rectangle(x0, ya, x1, yb, fill="#0f172a", outline="")

        for seg in self.model.allocated_segments_sorted():
            if seg.kind == "kernel":
                continue
            ya = self._kb_to_y(seg.start_kb)
            yb = self._kb_to_y(seg.end_kb)
            self.canvas.create_rectangle(x0, ya, x1, yb, fill=seg.color, outline="#0b1220", width=1)
            # Mostrar física y relativa (compacto)
            rel_txt = ""
            if seg.rel_start_kb is not None:
                rel_txt = f" | rel {seg.rel_start_kb}->{seg.rel_end_kb}"
            txt = f"{seg.label} ({seg.size_kb}KB){rel_txt}"
            self.canvas.create_text(x0 + 6, (ya + yb) / 2, anchor="w",
                                    text=txt, fill="#0b1220",
                                    font=("Segoe UI", 9, "bold"))

        self.canvas.create_text(380, 28, text="Bloque a ubicar (arrastrar →)",
                                fill="#e5e7eb", font=("Segoe UI", 10, "bold"))
        self.canvas.create_rectangle(340, 50, 500, 220, outline="#334155", width=2)
        self.canvas.create_text(420, 60, text="Zona de espera", fill="#94a3b8",
                                font=("Segoe UI", 9))

    def _update_stats(self):
        used = self.model.used_kb()
        free = self.model.free_kb()
        total = self.model.total_kb
        lives = self.state.lives
        self.lbl_stats.config(
            text=f"RAM: {used}/{total}KB usadas | Libre: {free}KB | Vidas: {lives}"
        )
        self.lbl_score.config(text=f"Puntaje: {self.state.score}")

        for item in self.tree.get_children():
            self.tree.delete(item)
        for pid, proc in sorted(self.state.processes.items()):
            mem = proc.mem_kb
            pct = (mem / total) * 100
            self.tree.insert("", "end", values=(pid, proc.name, mem, f"{pct:.1f}%"))

    def _log_event(self, app, level, msg):
        ts = time.strftime("%H:%M:%S")
        line = f"[{ts}] {app}: {level}: {msg}\n"
        self.console.insert("end", line, level if level in ("INFO", "WARN", "ERROR", "OK") else "INFO")
        self.console.see("end")

    # --------------- Pending request block ---------------

    def _spawn_pending_block(self, req: PendingRequest):
        if self.state.current_pending is not None:
            self.state.pending_queue.append(req)
            self._log_event(req.proc_name, "INFO",
                            f"Evento en cola: {req.kind.upper()} {req.size_kb}KB (PID {req.pid})")
            return

        self.state.current_pending = req
        self.lbl_pending.config(
            text=f"Evento actual: {req.kind.upper()} | PID {req.pid} | {req.size_kb}KB | Tiempo: {req.time_limit:.0f}s"
        )
        self.pb_time["maximum"] = req.time_limit * 1000
        self.pb_time["value"] = self.pb_time["maximum"]

        x0, y0, x1, y1 = 360, 90, 480, 190
        color = self.state.processes[req.pid].color if req.pid in self.state.processes else "#64748b"
        self.pending_block_id = self.canvas.create_rectangle(
            x0, y0, x1, y1, fill=color, outline="#e5e7eb", width=2, tags=("pending",)
        )
        self.pending_text_id = self.canvas.create_text(
            (x0 + x1) / 2, (y0 + y1) / 2,
            text=f"{req.label}\n{req.size_kb}KB",
            fill="#0b1220", font=("Segoe UI", 10, "bold"), tags=("pending",)
        )

        self.canvas.tag_bind("pending", "<ButtonPress-1>", self._on_drag_start)
        self.canvas.tag_bind("pending", "<B1-Motion>", self._on_drag_move)
        self.canvas.tag_bind("pending", "<ButtonRelease-1>", self._on_drag_end)

    def _clear_pending_block(self):
        if self.pending_block_id:
            self.canvas.delete(self.pending_block_id)
        if self.pending_text_id:
            self.canvas.delete(self.pending_text_id)
        self.pending_block_id = None
        self.pending_text_id = None
        self.state.current_pending = None
        self.lbl_pending.config(text="Evento actual: (ninguno)")
        self.pb_time["value"] = 0

    def _on_drag_start(self, event):
        if not self.pending_block_id:
            return
        x1, y1, x2, y2 = self.canvas.coords(self.pending_block_id)
        self.drag_offset = (event.x - x1, event.y - y1)

    def _on_drag_move(self, event):
        if not self.pending_block_id:
            return
        dx, dy = self.drag_offset
        new_x1 = event.x - dx
        new_y1 = event.y - dy

        x1, y1, x2, y2 = self.canvas.coords(self.pending_block_id)
        w = x2 - x1
        h = y2 - y1
        new_x2 = new_x1 + w
        new_y2 = new_y1 + h

        self.canvas.coords(self.pending_block_id, new_x1, new_y1, new_x2, new_y2)
        self.canvas.coords(self.pending_text_id, (new_x1 + new_x2) / 2, (new_y1 + new_y2) / 2)

        self._draw_drop_hint(new_x1, new_y1, new_x2, new_y2)

    def _draw_drop_hint(self, bx1, by1, bx2, by2):
        self.canvas.delete("hint")
        mx0, my0 = self.mem_x0, self.mem_y0
        mx1, my1 = mx0 + self.mem_w, my0 + self.mem_h
        if bx2 < mx0 or bx1 > mx1 or by2 < my0 or by1 > my1:
            return

        req = self.state.current_pending
        if not req:
            return

        start_kb = self._y_to_kb(by1)
        ok = self.model.can_allocate_at(start_kb, req.size_kb)

        shadow_y1 = self._kb_to_y(start_kb)
        shadow_y2 = self._kb_to_y(start_kb + req.size_kb)
        color = "#22c55e" if ok else "#ef4444"
        self.canvas.create_rectangle(mx0, shadow_y1, mx1, shadow_y2, outline=color, width=3, tags=("hint",))

    def _on_drag_end(self, event):
        if not self.pending_block_id:
            return
        self.canvas.delete("hint")

        req = self.state.current_pending
        if not req:
            return

        x1, y1, x2, y2 = self.canvas.coords(self.pending_block_id)
        mx0, my0 = self.mem_x0, self.mem_y0
        mx1, my1 = mx0 + self.mem_w, my0 + self.mem_h

        if x2 < mx0 or x1 > mx1 or y2 < my0 or y1 > my1:
            return  # sigue pendiente

        start_kb = self._y_to_kb(y1)
        if self.model.can_allocate_at(start_kb, req.size_kb):
            proc = self.state.processes.get(req.pid)
            if not proc:
                self._log_event(req.proc_name, "ERROR", f"PID {req.pid} no existe (terminado).")
                self._clear_pending_block()
                self._spawn_next_from_queue()
                return

            # --- NUEVO: asignación de dirección relativa (virtual) independiente de la física ---
            rel_base = proc.allocate_rel(req.size_kb)

            kind = "proc" if req.kind == "launch" else "dyn"
            label = f"PID {req.pid}" if kind == "proc" else f"PID {req.pid} malloc"
            seg = self.model.allocate(
                start_kb, req.size_kb, kind=kind, pid=req.pid,
                label=label, color=proc.color, rel_start_kb=rel_base
            )
            proc.segments.append(seg)
            proc.add_mapping(seg)

            elapsed = time.time() - req.created_at
            remaining = max(0.0, req.time_limit - elapsed)
            speed_bonus = int(100 * (remaining / req.time_limit))
            size_bonus = max(5, int(req.size_kb / max(1, self.model.total_kb) * 500))
            gained = speed_bonus + size_bonus
            self.state.score += gained

            self._log_event(req.proc_name, "OK",
                            f"Asignación exitosa: {req.kind.upper()} {req.size_kb}KB "
                            f"(rel @{rel_base}KB) → (phys @{start_kb}KB) (PID {req.pid}) +{gained} pts")

            self._clear_pending_block()
            self._redraw_memory()
            self._update_stats()
            self._spawn_next_from_queue()
        else:
            self._log_event(req.proc_name, "WARN",
                            f"No cabe ahí: se requiere hueco contiguo de {req.size_kb}KB (PID {req.pid})")

    def _spawn_next_from_queue(self):
        if self.state.current_pending is None and self.state.pending_queue:
            nxt = self.state.pending_queue.pop(0)
            self._spawn_pending_block(nxt)

    # --------------- Controls ---------------

    def _toggle_pause(self):
        self.state.running = not self.state.running
        self._log_event("systemd", "INFO", "Simulación reanudada" if self.state.running else "Simulación pausada")

    def _restart(self):
        if messagebox.askyesno("Reiniciar", "¿Deseas reiniciar la simulación?"):
            self.destroy()
            MemorySimApp().mainloop()

    def _simulate_launch(self):
        if not self.state.running:
            return

        pid = self.state.pid_counter
        self.state.pid_counter += 1
        name = random.choice(APP_NAMES)
        color = random.choice(PALETTE)

        self.state.processes[pid] = Process(pid=pid, name=name, color=color)

        total = self.model.total_kb
        req_kb = random.randint(max(256, total // 50), max(512, total // 8))
        req_kb = min(req_kb, max(256, self.model.free_kb()))

        self._log_event(name, "INFO", f"Lanzamiento: PID {pid}, solicita {req_kb}KB")
        req = PendingRequest(kind="launch", pid=pid, proc_name=name, size_kb=req_kb,
                             created_at=time.time(), time_limit=self.default_time_limit,
                             label=f"{name}\nPID {pid}")
        self._spawn_pending_block(req)
        self._update_stats()

    def _simulate_termination(self):
        if not self.state.running:
            return
        if not self.state.processes:
            self._log_event("bash", "WARN", "No hay procesos para terminar.")
            return

        sel = self.tree.selection()
        if sel:
            pid = int(self.tree.item(sel[0], "values")[0])
        else:
            pid = random.choice(list(self.state.processes.keys()))

        proc = self.state.processes.get(pid)
        if not proc:
            return

        freed = proc.mem_kb
        self.model.free_by_pid(pid)
        del self.state.processes[pid]

        self._log_event(proc.name, "INFO", f"Terminación: PID {pid}, libera {freed}KB")
        self._redraw_memory()
        self._update_stats()

        if self.state.current_pending and self.state.current_pending.pid == pid:
            self._log_event(proc.name, "WARN", "Evento pendiente cancelado por terminación del proceso.")
            self._clear_pending_block()
            self._spawn_next_from_queue()

        self.state.pending_queue = [r for r in self.state.pending_queue if r.pid != pid]

        # actualizar botón mapa
        self.btn_map.config(state="disabled")

    # --------------- NUEVO: UI Mapa de Memoria ---------------

    def _on_process_select(self, event=None):
        sel = self.tree.selection()
        self.btn_map.config(state="normal" if sel else "disabled")

    def _open_memory_map(self):
        sel = self.tree.selection()
        if not sel:
            return
        pid = int(self.tree.item(sel[0], "values")[0])
        proc = self.state.processes.get(pid)
        if not proc:
            return

        win = tk.Toplevel(self)
        win.title(f"Mapa de memoria — PID {pid} ({proc.name})")
        win.geometry("760x360")
        win.minsize(720, 320)

        info = ttk.Label(
            win,
            text=("Direcciones relativas (virtuales) crecen desde 0 de forma monotónica.\n"
                  "Las direcciones físicas pueden estar fragmentadas y no contiguas."),
            font=("Segoe UI", 10)
        )
        info.pack(anchor="w", padx=12, pady=(12, 6))

        frame = ttk.Frame(win)
        frame.pack(fill="both", expand=True, padx=12, pady=12)

        cols = ("Tipo", "Etiqueta", "RelStart (KB)", "RelEnd (KB)", "Size (KB)",
                "PhysStart (KB)", "PhysEnd (KB)")
        tv = ttk.Treeview(frame, columns=cols, show="headings")
        for c in cols:
            tv.heading(c, text=c)
            tv.column(c, width=105 if c not in ("Etiqueta",) else 160, anchor="center")
        tv.pack(fill="both", expand=True, side="left")

        sb = ttk.Scrollbar(frame, orient="vertical", command=tv.yview)
        sb.pack(side="right", fill="y")
        tv.configure(yscrollcommand=sb.set)

        # cargar mapeo
        for m in proc.mem_map:
            tv.insert(
                "", "end",
                values=(m.kind, m.label, m.rel_start_kb, m.rel_end_kb, m.size_kb,
                        m.phys_start_kb, m.phys_end_kb)
            )

        footer = ttk.Label(
            win,
            text=f"RelEnd máximo asignado (brk): {proc.next_rel_kb} KB  |  Segmentos activos: {len(proc.mem_map)}",
            font=("Segoe UI", 10, "bold")
        )
        footer.pack(anchor="w", padx=12, pady=(0, 12))

    # --------------- Simulation loops ---------------

    def _tick_game(self):
        if self.state.running and self.state.current_pending:
            req = self.state.current_pending
            elapsed = time.time() - req.created_at
            remaining = req.time_limit - elapsed
            self.pb_time["value"] = max(0, remaining * 1000)

            if remaining <= 0:
                self.state.lives -= 1
                self._log_event(req.proc_name, "ERROR",
                                f"Tiempo agotado: no se asignó {req.kind.upper()} de {req.size_kb}KB (PID {req.pid}). Pierdes 1 vida.")
                self._clear_pending_block()
                self._spawn_next_from_queue()
                self._update_stats()

                if self.state.lives <= 0:
                    self.state.running = False
                    self._log_event("systemd", "ERROR", "Juego terminado: sin vidas. Reinicia para intentar de nuevo.")
                    messagebox.showinfo("Fin del juego", "Sin vidas. Puedes reiniciar la simulación.")
        self.after(80, self._tick_game)

    def _tick_simulation_events(self):
        if self.state.running and self.model:
            if self.state.processes:
                choice = random.random()
                pid = random.choice(list(self.state.processes.keys()))
                proc = self.state.processes[pid]

                busy_factor = 0.35 if self.state.current_pending else 1.0

                if choice < 0.45 * busy_factor:
                    # solicitud dinámica: puede ubicarse en cualquier hueco físico (no adyacente al mismo PID)
                    total = self.model.total_kb
                    req_kb = random.randint(max(64, total // 200), max(128, total // 50))
                    req_kb = min(req_kb, max(64, self.model.free_kb()))
                    self._log_event(proc.name, "INFO", f"malloc(): PID {pid} solicita {req_kb}KB")
                    req = PendingRequest(kind="alloc", pid=pid, proc_name=proc.name, size_kb=req_kb,
                                         created_at=time.time(), time_limit=max(6, self.default_time_limit - 2),
                                         label=f"malloc()\nPID {pid}")
                    self._spawn_pending_block(req)

                elif choice < 0.70:
                    # free: libera un bloque dinámico cualquiera del PID (físico)
                    dyn = [s for s in proc.segments if s.kind == "dyn"]
                    if dyn:
                        seg = random.choice(dyn)
                        self.model.free_segment(seg)
                        proc.segments.remove(seg)

                        # eliminar del mapa por rel_start (IMPORTANTE: no reduce next_rel_kb)
                        if seg.rel_start_kb is not None:
                            proc.remove_mapping_by_rel_start(seg.rel_start_kb)

                        self._log_event(proc.name, "INFO",
                                        f"free(): PID {pid} libera {seg.size_kb}KB "
                                        f"(rel @{seg.rel_start_kb}KB) desde phys @{seg.start_kb}KB")
                        self._redraw_memory()
                        self._update_stats()

                elif choice < 0.80:
                    self._simulate_termination()
                else:
                    self._simulate_launch()
            else:
                if random.random() < 0.75:
                    self._simulate_launch()

            self._update_stats()

        self.after(random.randint(1800, 3200), self._tick_simulation_events)


if __name__ == "__main__":
    app = MemorySimApp()
    app.mainloop()