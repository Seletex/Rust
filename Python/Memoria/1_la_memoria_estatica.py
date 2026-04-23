#!/usr/bin/env python3
"""
MemOS - Simulador de Gestión de Memoria
Sistema educativo para cursos de Sistemas Operativos
"""

import tkinter as tk
from tkinter import ttk, messagebox, simpledialog
import random
import time
import threading
import math
from dataclasses import dataclass, field
from typing import Optional
from collections import OrderedDict
import colorsys

# ─────────────────────────────────────────────
#  COLORES Y TEMA
# ─────────────────────────────────────────────
THEME = {
    "bg":          "#0D0F1A",
    "panel":       "#13162B",
    "panel2":      "#1A1E35",
    "border":      "#2A2E4A",
    "accent":      "#00FFCC",
    "accent2":     "#FF6B35",
    "accent3":     "#A855F7",
    "text":        "#E8EAF6",
    "text_dim":    "#7B82B4",
    "kernel":      "#FF4444",
    "free":        "#1C2040",
    "success":     "#22C55E",
    "warning":     "#F59E0B",
    "danger":      "#EF4444",
    "grid":        "#1E2244",
}

PROCESS_COLORS = [
    "#00FFCC", "#FF6B35", "#A855F7", "#3B82F6", "#F59E0B",
    "#EC4899", "#10B981", "#F97316", "#8B5CF6", "#06B6D4",
    "#84CC16", "#EF4444", "#14B8A6", "#F43F5E", "#6366F1",
]

# ─────────────────────────────────────────────
#  MODELOS DE DATOS
# ─────────────────────────────────────────────
@dataclass
class MemoryBlock:
    start_kb: int
    size_kb: int
    pid: Optional[int] = None
    alloc_id: int = 0          # id dentro del proceso (para dinámica)
    is_dynamic: bool = False
    virtual_addr: int = 0      # dirección virtual relativa al proceso

@dataclass
class Process:
    pid: int
    name: str
    color: str
    static_kb: int
    blocks: list = field(default_factory=list)
    dynamic_allocs: list = field(default_factory=list)  # lista de MemoryBlock dinámicos
    next_virtual_addr: int = 0
    alive: bool = True

    @property
    def total_kb(self):
        return self.static_kb + sum(b.size_kb for b in self.dynamic_allocs)

    @property
    def dynamic_kb(self):
        return sum(b.size_kb for b in self.dynamic_allocs)


@dataclass
class Event:
    kind: str          # "launch" | "terminate" | "malloc" | "free"
    pid: int
    size_kb: int = 0
    alloc_id: int = 0
    name: str = ""
    timestamp: float = field(default_factory=time.time)

# ─────────────────────────────────────────────
#  MOTOR DE SIMULACIÓN
# ─────────────────────────────────────────────
class MemoryEngine:
    def __init__(self, total_kb: int, kernel_kb: int, mode: str):
        self.total_kb = total_kb
        self.kernel_kb = kernel_kb
        self.mode = mode          # "static" | "dynamic"
        self.user_start = kernel_kb
        self.user_kb = total_kb - kernel_kb

        # Tabla de memoria: lista ordenada de bloques
        self.blocks: list[MemoryBlock] = [
            MemoryBlock(0, kernel_kb, pid=-1),                        # kernel
            MemoryBlock(kernel_kb, total_kb - kernel_kb, pid=None),   # libre
        ]
        self.processes: dict[int, Process] = {}
        self._pid_counter = 100
        self._alloc_counter = 0
        self._color_index = 0
        self._lock = threading.Lock()

    # ── helpers ──────────────────────────────
    def _next_pid(self):
        self._pid_counter += 1
        return self._pid_counter

    def _next_color(self):
        c = PROCESS_COLORS[self._color_index % len(PROCESS_COLORS)]
        self._color_index += 1
        return c

    def free_kb(self):
        with self._lock:
            return sum(b.size_kb for b in self.blocks if b.pid is None)

    def used_kb(self):
        with self._lock:
            return sum(b.size_kb for b in self.blocks if b.pid is not None and b.pid != -1)

    # ── first-fit allocation ──────────────────
    def _find_free_region(self, size_kb: int) -> Optional[int]:
        """Devuelve el índice del bloque libre donde cabe size_kb, o None."""
        for i, b in enumerate(self.blocks):
            if b.pid is None and b.size_kb >= size_kb:
                return i
        return None

    def can_allocate(self, size_kb: int) -> bool:
        return self._find_free_region(size_kb) is not None

    def allocate(self, size_kb: int, pid: int, is_dynamic=False, alloc_id=0, virtual_addr=0) -> Optional[MemoryBlock]:
        """First-fit allocation. Retorna el bloque asignado o None."""
        with self._lock:
            idx = self._find_free_region(size_kb)
            if idx is None:
                return None
            free = self.blocks[idx]
            new_block = MemoryBlock(free.start_kb, size_kb, pid, alloc_id, is_dynamic, virtual_addr)
            remainder = free.size_kb - size_kb
            self.blocks[idx] = new_block
            if remainder > 0:
                self.blocks.insert(idx + 1, MemoryBlock(free.start_kb + size_kb, remainder))
            return new_block

    def free_block(self, block: MemoryBlock):
        with self._lock:
            for i, b in enumerate(self.blocks):
                if b.start_kb == block.start_kb and b.pid == block.pid:
                    self.blocks[i] = MemoryBlock(b.start_kb, b.size_kb, pid=None)
                    self._coalesce()
                    return True
        return False

    def _coalesce(self):
        """Une bloques libres adyacentes."""
        i = 0
        while i < len(self.blocks) - 1:
            if self.blocks[i].pid is None and self.blocks[i + 1].pid is None:
                merged = MemoryBlock(self.blocks[i].start_kb,
                                     self.blocks[i].size_kb + self.blocks[i + 1].size_kb)
                self.blocks[i] = merged
                self.blocks.pop(i + 1)
            else:
                i += 1

    # ── acciones de alto nivel ────────────────
    def launch_process(self, name: str, size_kb: int) -> Optional[Process]:
        pid = self._next_pid()
        color = self._next_color()
        proc = Process(pid, name, color, size_kb)
        self._alloc_counter += 1
        block = self.allocate(size_kb, pid, alloc_id=self._alloc_counter, virtual_addr=0)
        if block is None:
            return None
        proc.blocks.append(block)
        proc.next_virtual_addr = size_kb
        with self._lock:
            self.processes[pid] = proc
        return proc

    def terminate_process(self, pid: int) -> bool:
        with self._lock:
            proc = self.processes.get(pid)
            if not proc:
                return False
            proc.alive = False
            all_blocks = proc.blocks + proc.dynamic_allocs
        for b in all_blocks:
            self.free_block(b)
        with self._lock:
            del self.processes[pid]
        return True

    def malloc(self, pid: int, size_kb: int) -> Optional[MemoryBlock]:
        with self._lock:
            proc = self.processes.get(pid)
            if not proc or not proc.alive:
                return None
            virtual_addr = proc.next_virtual_addr
        self._alloc_counter += 1
        block = self.allocate(size_kb, pid, is_dynamic=True,
                              alloc_id=self._alloc_counter, virtual_addr=virtual_addr)
        if block is None:
            return None
        with self._lock:
            proc.dynamic_allocs.append(block)
            proc.next_virtual_addr += size_kb
        return block

    def mfree(self, pid: int, alloc_id: int) -> bool:
        with self._lock:
            proc = self.processes.get(pid)
            if not proc:
                return False
            target = next((b for b in proc.dynamic_allocs if b.alloc_id == alloc_id), None)
            if not target:
                return False
            proc.dynamic_allocs.remove(target)
        self.free_block(target)
        return True

    def snapshot(self):
        with self._lock:
            return list(self.blocks), dict(self.processes)


# ─────────────────────────────────────────────
#  GENERADOR DE EVENTOS
# ─────────────────────────────────────────────
PROGRAM_NAMES = [
    "Firefox", "VSCode", "Python3", "gcc", "bash", "nano",
    "htop", "curl", "ssh", "git", "docker", "nginx",
    "postgres", "redis", "node", "java", "rustc", "llvm",
]

class EventGenerator:
    def __init__(self, engine: MemoryEngine, callback, interval=(2.0, 5.0)):
        self.engine = engine
        self.callback = callback
        self.interval = interval
        self._running = False
        self._thread: Optional[threading.Thread] = None

    def start(self):
        self._running = True
        self._thread = threading.Thread(target=self._loop, daemon=True)
        self._thread.start()

    def stop(self):
        self._running = False

    def _loop(self):
        while self._running:
            delay = random.uniform(*self.interval)
            time.sleep(delay)
            if not self._running:
                break
            self._generate()

    def _generate(self):
        engine = self.engine
        pids = list(engine.processes.keys())

        # Decidir tipo de evento
        weights = ["launch"] * 5
        if pids:
            weights += ["terminate"] * 2
            if engine.mode == "dynamic":
                weights += ["malloc"] * 4
                weights += ["mfree"] * 2

        kind = random.choice(weights)

        if kind == "launch":
            max_possible = min(engine.free_kb(), engine.user_kb // 4)
            if max_possible < 16:
                return
            size_kb = random.randint(16, max(16, max_possible))
            name = random.choice(PROGRAM_NAMES)
            ev = Event("launch", 0, size_kb, name=name)
            self.callback(ev)

        elif kind == "terminate" and pids:
            pid = random.choice(pids)
            ev = Event("terminate", pid)
            self.callback(ev)

        elif kind == "malloc" and pids:
            pid = random.choice(pids)
            max_kb = min(engine.free_kb(), 256)
            if max_kb < 8:
                return
            size_kb = random.randint(8, max(8, max_kb))
            ev = Event("malloc", pid, size_kb)
            self.callback(ev)

        elif kind == "mfree" and pids:
            pid = random.choice(pids)
            proc = engine.processes.get(pid)
            if proc and proc.dynamic_allocs:
                alloc = random.choice(proc.dynamic_allocs)
                ev = Event("mfree", pid, alloc.size_kb, alloc_id=alloc.alloc_id)
                self.callback(ev)


# ─────────────────────────────────────────────
#  INTERFAZ GRÁFICA PRINCIPAL
# ─────────────────────────────────────────────
class MemOSApp(tk.Tk):
    def __init__(self):
        super().__init__()
        self.title("MemOS — Simulador de Gestión de Memoria")
        self.configure(bg=THEME["bg"])
        self.resizable(True, True)
        self.geometry("1400x900")
        self._setup_styles()
        self._show_config_dialog()

    def _setup_styles(self):
        style = ttk.Style(self)
        style.theme_use("clam")
        style.configure("TFrame", background=THEME["bg"])
        style.configure("Panel.TFrame", background=THEME["panel"])
        style.configure("TLabel",
                        background=THEME["bg"],
                        foreground=THEME["text"],
                        font=("Consolas", 10))
        style.configure("Title.TLabel",
                        background=THEME["bg"],
                        foreground=THEME["accent"],
                        font=("Consolas", 13, "bold"))
        style.configure("Dim.TLabel",
                        background=THEME["bg"],
                        foreground=THEME["text_dim"],
                        font=("Consolas", 9))
        style.configure("TButton",
                        background=THEME["panel2"],
                        foreground=THEME["text"],
                        font=("Consolas", 10),
                        borderwidth=0,
                        relief="flat")
        style.map("TButton",
                  background=[("active", THEME["accent"]),
                               ("pressed", THEME["accent"])],
                  foreground=[("active", THEME["bg"])])
        style.configure("Accent.TButton",
                        background=THEME["accent"],
                        foreground=THEME["bg"],
                        font=("Consolas", 10, "bold"),
                        borderwidth=0)
        style.configure("TScrollbar",
                        background=THEME["panel2"],
                        troughcolor=THEME["panel"],
                        arrowcolor=THEME["text_dim"])
        style.configure("TNotebook",
                        background=THEME["bg"],
                        tabmargins=[0, 0, 0, 0])
        style.configure("TNotebook.Tab",
                        background=THEME["panel2"],
                        foreground=THEME["text_dim"],
                        font=("Consolas", 10),
                        padding=[12, 5])
        style.map("TNotebook.Tab",
                  background=[("selected", THEME["panel"])],
                  foreground=[("selected", THEME["accent"])])

    def _show_config_dialog(self):
        dlg = ConfigDialog(self)
        self.wait_window(dlg)
        if not hasattr(dlg, 'result') or dlg.result is None:
            self.destroy()
            return
        cfg = dlg.result
        self._build_main_ui(cfg)

    def _build_main_ui(self, cfg):
        total_kb   = cfg["total_kb"]
        kernel_kb  = cfg["kernel_kb"]
        mode       = cfg["mode"]

        self.engine = MemoryEngine(total_kb, kernel_kb, mode)
        self.mode = mode
        self.score = 0
        self.lives = 3
        self.pending_events: list[Event] = []
        self.active_drag = None           # evento pendiente de asignación manual
        self.drag_timer_id = None
        self.drag_deadline = 0
        self.TIME_LIMIT = 15              # segundos para asignar

        # ── Layout principal ──
        top = ttk.Frame(self, style="TFrame")
        top.pack(fill="x", padx=10, pady=(8, 0))
        self._build_header(top, total_kb, kernel_kb, mode)

        main = ttk.Frame(self, style="TFrame")
        main.pack(fill="both", expand=True, padx=10, pady=8)
        main.columnconfigure(0, weight=3)
        main.columnconfigure(1, weight=2)
        main.columnconfigure(2, weight=2)
        main.rowconfigure(0, weight=1)

        # Panel izquierdo: RAM visual
        self._build_ram_panel(main)
        # Panel central: consola de eventos
        self._build_console_panel(main)
        # Panel derecho: lista de procesos
        self._build_process_panel(main)

        # ── Barra inferior ──
        bot = ttk.Frame(self, style="TFrame")
        bot.pack(fill="x", padx=10, pady=(0, 8))
        self._build_status_bar(bot)

        # ── Generador de eventos ──
        self.gen = EventGenerator(self.engine, self._on_event, interval=(3.0, 7.0))
        self.gen.start()

        # ── Bucle de refresco ──
        self._refresh()

    # ─────────────────────────────────────────
    #  HEADER
    # ─────────────────────────────────────────
    def _build_header(self, parent, total_kb, kernel_kb, mode):
        ttk.Label(parent,
                  text="◈ MemOS",
                  font=("Consolas", 20, "bold"),
                  foreground=THEME["accent"],
                  background=THEME["bg"]).pack(side="left")
        ttk.Label(parent,
                  text=f"  Simulador de Gestión de Memoria  │  RAM: {total_kb} KB  │  Kernel: {kernel_kb} KB  │  Modo: {'Estático' if mode=='static' else 'Dinámico'}",
                  font=("Consolas", 10),
                  foreground=THEME["text_dim"],
                  background=THEME["bg"]).pack(side="left", pady=5)

        # Botón pausa / reanudar
        self._paused = False
        self._btn_pause = ttk.Button(parent, text="⏸ Pausar",
                                     command=self._toggle_pause)
        self._btn_pause.pack(side="right", padx=4)
        ttk.Button(parent, text="↺ Reiniciar",
                   command=self._restart).pack(side="right", padx=4)

    # ─────────────────────────────────────────
    #  PANEL RAM
    # ─────────────────────────────────────────
    def _build_ram_panel(self, parent):
        frame = tk.Frame(parent, bg=THEME["panel"],
                         highlightbackground=THEME["border"],
                         highlightthickness=1)
        frame.grid(row=0, column=0, sticky="nsew", padx=(0, 6))

        ttk.Label(frame, text="▸ MAPA DE MEMORIA RAM",
                  style="Title.TLabel",
                  background=THEME["panel"]).pack(anchor="w", padx=10, pady=(8, 4))

        # Canvas RAM
        self.ram_canvas = tk.Canvas(frame,
                                    bg=THEME["free"],
                                    highlightthickness=0,
                                    cursor="crosshair")
        self.ram_canvas.pack(fill="both", expand=True, padx=10, pady=(0, 10))

        # Zona de arrastre (para el bloque pendiente)
        self.drag_frame = tk.Frame(frame, bg=THEME["panel"])
        self.drag_frame.pack(fill="x", padx=10, pady=(0, 6))
        self.drag_info_label = tk.Label(self.drag_frame, text="",
                                        bg=THEME["panel"],
                                        fg=THEME["accent2"],
                                        font=("Consolas", 10, "bold"))
        self.drag_info_label.pack(side="left")
        self.timer_label = tk.Label(self.drag_frame, text="",
                                    bg=THEME["panel"],
                                    fg=THEME["warning"],
                                    font=("Consolas", 11, "bold"))
        self.timer_label.pack(side="right")

        # Botón de asignación automática (ayuda)
        self.btn_auto = ttk.Button(frame, text="⚡ Auto-asignar (−10 pts)",
                                   command=self._auto_assign)
        self.btn_auto.pack(pady=(0, 8))
        self.btn_auto.pack_forget()  # oculto por defecto

        # drag state
        self._drag_block = None
        self._drag_x = 0
        self._drag_y = 0
        self.ram_canvas.bind("<ButtonPress-1>", self._drag_start)
        self.ram_canvas.bind("<B1-Motion>", self._drag_motion)
        self.ram_canvas.bind("<ButtonRelease-1>", self._drag_release)

    # ─────────────────────────────────────────
    #  PANEL CONSOLA
    # ─────────────────────────────────────────
    def _build_console_panel(self, parent):
        frame = tk.Frame(parent, bg=THEME["panel"],
                         highlightbackground=THEME["border"],
                         highlightthickness=1)
        frame.grid(row=0, column=1, sticky="nsew", padx=3)

        ttk.Label(frame, text="▸ CONSOLA DE EVENTOS",
                  style="Title.TLabel",
                  background=THEME["panel"]).pack(anchor="w", padx=10, pady=(8, 4))

        self.console_text = tk.Text(frame,
                                    bg=THEME["bg"],
                                    fg=THEME["text"],
                                    font=("Consolas", 9),
                                    state="disabled",
                                    wrap="word",
                                    relief="flat",
                                    borderwidth=0,
                                    insertbackground=THEME["accent"])
        sb = ttk.Scrollbar(frame, command=self.console_text.yview)
        self.console_text.configure(yscrollcommand=sb.set)
        sb.pack(side="right", fill="y")
        self.console_text.pack(fill="both", expand=True, padx=6, pady=(0, 6))

        # Tags de color
        self.console_text.tag_configure("launch",    foreground=THEME["success"])
        self.console_text.tag_configure("terminate", foreground=THEME["danger"])
        self.console_text.tag_configure("malloc",    foreground=THEME["accent3"])
        self.console_text.tag_configure("mfree",     foreground=THEME["warning"])
        self.console_text.tag_configure("system",    foreground=THEME["text_dim"])
        self.console_text.tag_configure("score",     foreground=THEME["accent"])
        self.console_text.tag_configure("error",     foreground=THEME["danger"])
        self.console_text.tag_configure("time",      foreground=THEME["text_dim"])

    # ─────────────────────────────────────────
    #  PANEL PROCESOS
    # ─────────────────────────────────────────
    def _build_process_panel(self, parent):
        frame = tk.Frame(parent, bg=THEME["panel"],
                         highlightbackground=THEME["border"],
                         highlightthickness=1)
        frame.grid(row=0, column=2, sticky="nsew", padx=(6, 0))

        ttk.Label(frame, text="▸ TABLA DE PROCESOS",
                  style="Title.TLabel",
                  background=THEME["panel"]).pack(anchor="w", padx=10, pady=(8, 4))

        cols = ("PID", "Nombre", "Est. KB", "Din. KB", "Total", "%")
        self.proc_tree = ttk.Treeview(frame, columns=cols, show="headings",
                                      height=15,
                                      selectmode="browse")
        widths = [55, 80, 65, 65, 65, 55]
        for col, w in zip(cols, widths):
            self.proc_tree.heading(col, text=col)
            self.proc_tree.column(col, width=w, anchor="center")

        style = ttk.Style()
        style.configure("Treeview",
                         background=THEME["bg"],
                         foreground=THEME["text"],
                         fieldbackground=THEME["bg"],
                         font=("Consolas", 9),
                         rowheight=22)
        style.configure("Treeview.Heading",
                         background=THEME["panel2"],
                         foreground=THEME["accent"],
                         font=("Consolas", 9, "bold"))
        style.map("Treeview", background=[("selected", THEME["accent3"])])

        vsb = ttk.Scrollbar(frame, orient="vertical", command=self.proc_tree.yview)
        self.proc_tree.configure(yscrollcommand=vsb.set)
        vsb.pack(side="right", fill="y")
        self.proc_tree.pack(fill="both", expand=True, padx=6, pady=(0, 6))

        if self.mode == "dynamic":
            ttk.Button(frame, text="🗺  Ver Mapa de Memoria",
                       command=self._show_memory_map).pack(pady=(0, 6))

        # Terminar proceso manualmente
        ttk.Button(frame, text="✕ Terminar proceso seleccionado",
                   command=self._kill_selected).pack(pady=(0, 8))

    # ─────────────────────────────────────────
    #  BARRA DE ESTADO
    # ─────────────────────────────────────────
    def _build_status_bar(self, parent):
        self.lbl_score  = tk.Label(parent, text="PUNTOS: 0",
                                   bg=THEME["bg"], fg=THEME["accent"],
                                   font=("Consolas", 12, "bold"))
        self.lbl_score.pack(side="left", padx=10)

        self.lbl_lives  = tk.Label(parent, text="♥ ♥ ♥",
                                   bg=THEME["bg"], fg=THEME["danger"],
                                   font=("Consolas", 13))
        self.lbl_lives.pack(side="left", padx=10)

        self.lbl_free   = tk.Label(parent, text="",
                                   bg=THEME["bg"], fg=THEME["text_dim"],
                                   font=("Consolas", 10))
        self.lbl_free.pack(side="left", padx=10)

        self.lbl_frag   = tk.Label(parent, text="",
                                   bg=THEME["bg"], fg=THEME["text_dim"],
                                   font=("Consolas", 10))
        self.lbl_frag.pack(side="left", padx=10)

    # ─────────────────────────────────────────
    #  RENDER RAM CANVAS
    # ─────────────────────────────────────────
    def _render_ram(self):
        c = self.ram_canvas
        c.delete("all")
        w = c.winfo_width()
        h = c.winfo_height()
        if w < 10 or h < 10:
            return

        blocks, procs = self.engine.snapshot()
        total = self.engine.total_kb

        # Fondo de cuadrícula
        c.create_rectangle(0, 0, w, h, fill=THEME["free"], outline="")
        grid_step = max(5, h // 20)
        for y in range(0, h, grid_step):
            c.create_line(0, y, w, y, fill=THEME["grid"], width=1)

        for b in blocks:
            y0 = int(b.start_kb / total * h)
            y1 = int((b.start_kb + b.size_kb) / total * h)
            if y1 <= y0:
                y1 = y0 + 1

            if b.pid == -1:
                # Kernel
                c.create_rectangle(2, y0, w - 2, y1,
                                    fill=THEME["kernel"],
                                    outline=THEME["danger"],
                                    width=1)
                c.create_text(w // 2, (y0 + y1) // 2,
                              text=f"KERNEL  {b.size_kb} KB",
                              fill="white",
                              font=("Consolas", 9, "bold"))
            elif b.pid is None:
                # Libre
                c.create_rectangle(2, y0, w - 2, y1,
                                    fill=THEME["free"],
                                    outline=THEME["border"],
                                    width=1, dash=(4, 4))
                if y1 - y0 > 14:
                    c.create_text(w // 2, (y0 + y1) // 2,
                                  text=f"LIBRE  {b.size_kb} KB",
                                  fill=THEME["text_dim"],
                                  font=("Consolas", 8))
            else:
                proc = procs.get(b.pid)
                color = proc.color if proc else "#666"
                c.create_rectangle(2, y0, w - 2, y1,
                                    fill=color,
                                    outline=self._darken(color),
                                    width=1)
                label = f"PID {b.pid}"
                if proc:
                    label = f"{proc.name} [{b.pid}]"
                if b.is_dynamic:
                    label += " dyn"
                label += f"  {b.size_kb} KB"
                if y1 - y0 > 13:
                    c.create_text(w // 2, (y0 + y1) // 2,
                                  text=label,
                                  fill=self._contrast_text(color),
                                  font=("Consolas", 8, "bold"))

        # Regla de KB
        step_kb = max(1, total // 20)
        for kb in range(0, total + 1, step_kb):
            y = int(kb / total * h)
            c.create_line(0, y, 8, y, fill=THEME["text_dim"], width=1)
            if kb % (step_kb * 2) == 0:
                c.create_text(10, y, text=f"{kb}", anchor="w",
                              fill=THEME["text_dim"],
                              font=("Consolas", 7))

        # Resaltar zona de drop si hay bloque pendiente
        if self.active_drag:
            c.create_rectangle(0, 0, w, h,
                                outline=THEME["accent"],
                                width=2, dash=(6, 3))

    # ─────────────────────────────────────────
    #  DRAG & DROP
    # ─────────────────────────────────────────
    def _drag_start(self, event):
        """Inicia arrastre visual si hay evento pendiente."""
        if not self.active_drag:
            return
        self._drag_x = event.x
        self._drag_y = event.y

    def _drag_motion(self, event):
        if not self.active_drag:
            return
        c = self.ram_canvas
        c.delete("drag_preview")
        x, y = event.x, event.y
        c.create_rectangle(x - 60, y - 15, x + 60, y + 15,
                            fill=THEME["accent2"], outline=THEME["accent"],
                            stipple="gray50", tags="drag_preview", width=2)
        ev = self.active_drag
        c.create_text(x, y, text=f"{ev.size_kb} KB",
                      fill="white", font=("Consolas", 9, "bold"),
                      tags="drag_preview")

    def _drag_release(self, event):
        if not self.active_drag:
            return
        c = self.ram_canvas
        c.delete("drag_preview")
        self._try_assign_at(event.y)

    def _try_assign_at(self, canvas_y: int):
        """Intenta asignar el bloque pendiente en la posición Y del canvas."""
        ev = self.active_drag
        if not ev:
            return
        h = self.ram_canvas.winfo_height()
        target_kb = int(canvas_y / h * self.engine.total_kb)
        # Comprobamos si hay un bloque libre que cubra target_kb
        blocks, _ = self.engine.snapshot()
        target_block = None
        for b in blocks:
            if b.pid is None and b.start_kb <= target_kb < b.start_kb + b.size_kb:
                if b.size_kb >= ev.size_kb:
                    target_block = b
                    break

        if target_block:
            self._complete_launch(ev)
        else:
            self._miss_assign()

    def _complete_launch(self, ev: Event):
        """Asigna exitosamente el proceso pendiente."""
        elapsed = self.TIME_LIMIT - max(0, self.drag_deadline - time.time())
        speed_bonus = max(0, int((self.TIME_LIMIT - elapsed) * 5))
        points = 100 + speed_bonus
        self.score += points
        self._cancel_drag_timer()
        self.active_drag = None
        self.drag_info_label.config(text="")
        self.timer_label.config(text="")
        self.btn_auto.pack_forget()

        proc = self.engine.launch_process(ev.name, ev.size_kb)
        if proc:
            self._log(f"[✓] PID {proc.pid} ({ev.name}) asignado — +{points} pts", "score")
        else:
            self._log("[!] Sin memoria suficiente tras la asignación.", "error")
        self._update_score()

    def _miss_assign(self):
        """El estudiante soltó en un lugar inválido."""
        self.lives -= 1
        self._log(f"[✗] Asignación fallida — Vidas restantes: {self.lives}", "error")
        self._update_lives()
        if self.lives <= 0:
            self._game_over()

    def _auto_assign(self):
        """Asignación automática con penalización."""
        ev = self.active_drag
        if not ev:
            return
        self.score = max(0, self.score - 10)
        self._cancel_drag_timer()
        self.active_drag = None
        self.drag_info_label.config(text="")
        self.timer_label.config(text="")
        self.btn_auto.pack_forget()
        proc = self.engine.launch_process(ev.name, ev.size_kb)
        if proc:
            self._log(f"[⚡] PID {proc.pid} ({ev.name}) auto-asignado — −10 pts", "system")
        self._update_score()

    def _cancel_drag_timer(self):
        if self.drag_timer_id:
            self.after_cancel(self.drag_timer_id)
            self.drag_timer_id = None

    def _start_drag_timer(self):
        self.drag_deadline = time.time() + self.TIME_LIMIT
        self._tick_drag_timer()

    def _tick_drag_timer(self):
        if not self.active_drag:
            return
        remaining = self.drag_deadline - time.time()
        if remaining <= 0:
            self._timeout()
            return
        color = THEME["success"] if remaining > 8 else \
                THEME["warning"] if remaining > 4 else THEME["danger"]
        self.timer_label.config(text=f"⏱ {remaining:.1f}s", fg=color)
        self.drag_timer_id = self.after(100, self._tick_drag_timer)

    def _timeout(self):
        ev = self.active_drag
        self.active_drag = None
        self.drag_info_label.config(text="")
        self.timer_label.config(text="")
        self.btn_auto.pack_forget()
        self.lives -= 1
        self._log(f"[⏰] Tiempo agotado para asignar {ev.size_kb} KB — Vidas: {self.lives}", "error")
        self._update_lives()
        if self.lives <= 0:
            self._game_over()

    # ─────────────────────────────────────────
    #  MANEJO DE EVENTOS
    # ─────────────────────────────────────────
    def _on_event(self, ev: Event):
        """Callback del generador. Ejecutado en hilo de fondo → scheudled en UI."""
        self.after(0, lambda: self._handle_event(ev))

    def _handle_event(self, ev: Event):
        if self._paused:
            return
        ts = time.strftime("%H:%M:%S")

        if ev.kind == "launch":
            if not self.engine.can_allocate(ev.size_kb):
                self._log(f"[{ts}] LANZAR {ev.name}: {ev.size_kb} KB — SIN MEMORIA", "error")
                return
            # Poner en cola para asignación manual
            if self.active_drag:
                # Ya hay un bloque pendiente; auto-asignar el anterior con penalización
                self._auto_assign()
            self.active_drag = ev
            self.drag_info_label.config(
                text=f"◈ NUEVA TAREA: {ev.name}  [{ev.size_kb} KB]  — arrastra al mapa")
            self.btn_auto.pack(pady=(0, 8))
            self._start_drag_timer()
            self._log(f"[{ts}] ▶ LANZAR  {ev.name}  PID pendiente  {ev.size_kb} KB", "launch")

        elif ev.kind == "terminate":
            name = self.engine.processes.get(ev.pid, Process(ev.pid, "?", "", 0)).name
            ok = self.engine.terminate_process(ev.pid)
            if ok:
                self._log(f"[{ts}] ■ FIN     PID {ev.pid} ({name})", "terminate")

        elif ev.kind == "malloc":
            proc = self.engine.processes.get(ev.pid)
            if not proc:
                return
            block = self.engine.malloc(ev.pid, ev.size_kb)
            if block:
                self._log(f"[{ts}] ⊕ MALLOC  PID {ev.pid} ({proc.name})  +{ev.size_kb} KB  "
                          f"@virt 0x{block.virtual_addr:04X}  @real {block.start_kb} KB", "malloc")
            else:
                self._log(f"[{ts}] ✗ MALLOC  PID {ev.pid} — sin memoria dinámica", "error")

        elif ev.kind == "mfree":
            proc = self.engine.processes.get(ev.pid)
            if not proc:
                return
            ok = self.engine.mfree(ev.pid, ev.alloc_id)
            if ok:
                self._log(f"[{ts}] ⊖ FREE    PID {ev.pid} ({proc.name})  −{ev.size_kb} KB", "mfree")

    # ─────────────────────────────────────────
    #  HELPERS UI
    # ─────────────────────────────────────────
    def _log(self, msg: str, tag: str = "system"):
        self.console_text.configure(state="normal")
        self.console_text.insert("end", msg + "\n", tag)
        self.console_text.see("end")
        self.console_text.configure(state="disabled")

    def _update_score(self):
        self.lbl_score.config(text=f"PUNTOS: {self.score}")

    def _update_lives(self):
        hearts = "♥ " * self.lives + "♡ " * (3 - self.lives)
        self.lbl_lives.config(text=hearts.strip())

    def _update_proc_table(self):
        _, procs = self.engine.snapshot()
        # Limpiar
        for item in self.proc_tree.get_children():
            self.proc_tree.delete(item)
        for proc in sorted(procs.values(), key=lambda p: p.pid):
            pct = proc.total_kb / self.engine.total_kb * 100
            self.proc_tree.insert("", "end",
                                  iid=str(proc.pid),
                                  values=(proc.pid, proc.name,
                                          proc.static_kb,
                                          proc.dynamic_kb,
                                          proc.total_kb,
                                          f"{pct:.1f}%"),
                                  tags=(proc.color,))
            self.proc_tree.tag_configure(proc.color, foreground=proc.color)

    def _update_status(self):
        free = self.engine.free_kb()
        used = self.engine.used_kb()
        total = self.engine.total_kb
        pct_used = used / total * 100
        self.lbl_free.config(
            text=f"Libre: {free} KB  |  Usado: {used} KB  |  {pct_used:.1f}% ocupado")
        # Fragmentación externa
        blocks, _ = self.engine.snapshot()
        free_blocks = [b for b in blocks if b.pid is None]
        frag = (1 - max((b.size_kb for b in free_blocks), default=0) / free) * 100 if free > 0 else 0
        self.lbl_frag.config(text=f"Fragmentación ext: {frag:.1f}%")

    def _refresh(self):
        self._render_ram()
        self._update_proc_table()
        self._update_status()
        self.after(250, self._refresh)

    def _toggle_pause(self):
        self._paused = not self._paused
        self._btn_pause.config(text="▶ Reanudar" if self._paused else "⏸ Pausar")
        if self._paused:
            self.gen.stop()
        else:
            self.gen.start()

    def _restart(self):
        if messagebox.askyesno("Reiniciar", "¿Reiniciar la simulación?"):
            self.gen.stop()
            self.destroy()
            app = MemOSApp()
            app.mainloop()

    def _kill_selected(self):
        sel = self.proc_tree.selection()
        if not sel:
            messagebox.showinfo("Info", "Selecciona un proceso primero.")
            return
        pid = int(sel[0])
        ok = self.engine.terminate_process(pid)
        if ok:
            self._log(f"[MANUAL] PID {pid} terminado por el usuario.", "terminate")

    def _game_over(self):
        self.gen.stop()
        messagebox.showerror("GAME OVER",
                             f"¡Sin vidas!\nPuntuación final: {self.score} puntos")
        self.destroy()

    # ─────────────────────────────────────────
    #  MAPA DE MEMORIA (modo dinámico)
    # ─────────────────────────────────────────
    def _show_memory_map(self):
        sel = self.proc_tree.selection()
        if not sel:
            messagebox.showinfo("Info", "Selecciona un proceso primero.")
            return
        pid = int(sel[0])
        _, procs = self.engine.snapshot()
        proc = procs.get(pid)
        if not proc:
            messagebox.showinfo("Info", "Proceso no encontrado.")
            return
        MemoryMapWindow(self, proc)

    # ─────────────────────────────────────────
    #  UTILIDADES DE COLOR
    # ─────────────────────────────────────────
    @staticmethod
    def _darken(hex_color: str, factor=0.6) -> str:
        hex_color = hex_color.lstrip("#")
        r, g, b = int(hex_color[0:2], 16), int(hex_color[2:4], 16), int(hex_color[4:6], 16)
        return f"#{int(r*factor):02X}{int(g*factor):02X}{int(b*factor):02X}"

    @staticmethod
    def _contrast_text(hex_color: str) -> str:
        hex_color = hex_color.lstrip("#")
        r, g, b = int(hex_color[0:2], 16), int(hex_color[2:4], 16), int(hex_color[4:6], 16)
        luminance = 0.299 * r + 0.587 * g + 0.114 * b
        return "#000000" if luminance > 140 else "#FFFFFF"


# ─────────────────────────────────────────────
#  DIÁLOGO DE CONFIGURACIÓN INICIAL
# ─────────────────────────────────────────────
class ConfigDialog(tk.Toplevel):
    def __init__(self, parent):
        super().__init__(parent)
        self.title("MemOS — Configuración Inicial")
        self.configure(bg=THEME["bg"])
        self.resizable(False, False)
        self.result = None
        self.grab_set()
        self._build()
        self.geometry("520x500")
        self.protocol("WM_DELETE_WINDOW", self._cancel)

    def _build(self):
        # Título
        tk.Label(self, text="◈ MemOS",
                 bg=THEME["bg"], fg=THEME["accent"],
                 font=("Consolas", 22, "bold")).pack(pady=(20, 4))
        tk.Label(self, text="Simulador de Gestión de Memoria",
                 bg=THEME["bg"], fg=THEME["text_dim"],
                 font=("Consolas", 11)).pack()

        sep = tk.Frame(self, bg=THEME["border"], height=1)
        sep.pack(fill="x", padx=30, pady=16)

        # ── Opciones ──
        form = tk.Frame(self, bg=THEME["bg"])
        form.pack(padx=40)

        def row(label, widget_factory, default):
            f = tk.Frame(form, bg=THEME["bg"])
            f.pack(fill="x", pady=6)
            tk.Label(f, text=label, bg=THEME["bg"], fg=THEME["text"],
                     font=("Consolas", 10), width=28, anchor="w").pack(side="left")
            var = tk.IntVar(value=default)
            widget = widget_factory(f, var)
            widget.pack(side="left", fill="x", expand=True)
            return var

        # RAM total
        def make_ram(parent, var):
            f = tk.Frame(parent, bg=THEME["bg"])
            scale = tk.Scale(f, from_=512, to=8192, resolution=128,
                             orient="horizontal", variable=var,
                             bg=THEME["bg"], fg=THEME["text"],
                             troughcolor=THEME["panel2"],
                             highlightthickness=0,
                             activebackground=THEME["accent"])
            scale.pack(side="left", fill="x", expand=True)
            lbl = tk.Label(f, textvariable=var,
                           bg=THEME["bg"], fg=THEME["accent"],
                           font=("Consolas", 10), width=6)
            lbl.pack(side="left")
            tk.Label(f, text=" KB", bg=THEME["bg"], fg=THEME["text_dim"],
                     font=("Consolas", 9)).pack(side="left")
            return f

        self.var_ram    = tk.IntVar(value=2048)
        self.var_kernel = tk.IntVar(value=256)

        row("RAM Total (KB):", make_ram, 2048)

        def make_kernel(parent, var):
            f = tk.Frame(parent, bg=THEME["bg"])
            scale = tk.Scale(f, from_=64, to=1024, resolution=64,
                             orient="horizontal", variable=var,
                             bg=THEME["bg"], fg=THEME["text"],
                             troughcolor=THEME["panel2"],
                             highlightthickness=0,
                             activebackground=THEME["kernel"])
            scale.pack(side="left", fill="x", expand=True)
            lbl = tk.Label(f, textvariable=var,
                           bg=THEME["bg"], fg=THEME["kernel"],
                           font=("Consolas", 10), width=6)
            lbl.pack(side="left")
            tk.Label(f, text=" KB", bg=THEME["bg"], fg=THEME["text_dim"],
                     font=("Consolas", 9)).pack(side="left")
            return f

        # Usar el mismo var_ram ya creado
        f2 = tk.Frame(form, bg=THEME["bg"])
        f2.pack(fill="x", pady=6)
        tk.Label(f2, text="Memoria Kernel (KB):", bg=THEME["bg"], fg=THEME["text"],
                 font=("Consolas", 10), width=28, anchor="w").pack(side="left")
        make_kernel(f2, self.var_kernel).pack(side="left", fill="x", expand=True)

        # Conectar sliders al var correcto
        def update_ram(val):
            self.var_ram.set(int(val))
        def update_kernel(val):
            self.var_kernel.set(int(val))

        # Modo
        sep2 = tk.Frame(self, bg=THEME["border"], height=1)
        sep2.pack(fill="x", padx=30, pady=12)

        tk.Label(self, text="Modo de Simulación:",
                 bg=THEME["bg"], fg=THEME["text"],
                 font=("Consolas", 11, "bold")).pack()

        self.var_mode = tk.StringVar(value="static")
        modes = tk.Frame(self, bg=THEME["bg"])
        modes.pack(pady=8)

        def mode_btn(text, value, desc):
            f = tk.Frame(modes, bg=THEME["panel2"],
                         highlightbackground=THEME["border"],
                         highlightthickness=1)
            f.pack(side="left", padx=10, pady=4, ipadx=8, ipady=6)
            tk.Radiobutton(f, text=text, variable=self.var_mode, value=value,
                           bg=THEME["panel2"], fg=THEME["accent"],
                           selectcolor=THEME["panel"],
                           activebackground=THEME["panel2"],
                           font=("Consolas", 10, "bold")).pack()
            tk.Label(f, text=desc, bg=THEME["panel2"], fg=THEME["text_dim"],
                     font=("Consolas", 8), wraplength=180).pack()

        mode_btn("① Memoria Estática",  "static",
                 "Procesos con tamaño fijo.\nArrastra bloques al mapa.")
        mode_btn("② Memoria Dinámica",  "dynamic",
                 "malloc/free en tiempo real.\nMapa de memoria virtual ↔ real.")

        sep3 = tk.Frame(self, bg=THEME["border"], height=1)
        sep3.pack(fill="x", padx=30, pady=12)

        tk.Button(self, text="▶  INICIAR SIMULACIÓN",
                  bg=THEME["accent"], fg=THEME["bg"],
                  font=("Consolas", 12, "bold"),
                  relief="flat", padx=20, pady=8,
                  cursor="hand2",
                  command=self._ok).pack(pady=4)

    def _ok(self):
        ram = self.var_ram.get()
        kern = self.var_kernel.get()
        if kern >= ram:
            messagebox.showerror("Error", "El kernel no puede usar toda la RAM.")
            return
        self.result = {
            "total_kb":  ram,
            "kernel_kb": kern,
            "mode":      self.var_mode.get(),
        }
        self.destroy()

    def _cancel(self):
        self.result = None
        self.destroy()


# ─────────────────────────────────────────────
#  VENTANA MAPA DE MEMORIA
# ─────────────────────────────────────────────
class MemoryMapWindow(tk.Toplevel):
    def __init__(self, parent, proc: Process):
        super().__init__(parent)
        self.title(f"Mapa de Memoria — PID {proc.pid} ({proc.name})")
        self.configure(bg=THEME["bg"])
        self.geometry("700x500")
        self._build(proc)

    def _build(self, proc: Process):
        tk.Label(self, text=f"◈ Mapa de Memoria Virtual ↔ Real",
                 bg=THEME["bg"], fg=THEME["accent"],
                 font=("Consolas", 13, "bold")).pack(anchor="w", padx=14, pady=(10, 2))
        tk.Label(self,
                 text=f"PID {proc.pid} | {proc.name} | Estático: {proc.static_kb} KB | "
                      f"Dinámico: {proc.dynamic_kb} KB | Total: {proc.total_kb} KB",
                 bg=THEME["bg"], fg=THEME["text_dim"],
                 font=("Consolas", 9)).pack(anchor="w", padx=14)

        sep = tk.Frame(self, bg=THEME["border"], height=1)
        sep.pack(fill="x", padx=14, pady=8)

        # Tabla
        cols = ("Segmento", "Tipo", "Dir. Virt. Inicio", "Dir. Virt. Fin",
                "Dir. Real (KB)", "Tamaño (KB)")
        tree = ttk.Treeview(self, columns=cols, show="headings", height=15)
        widths = [80, 70, 130, 130, 120, 100]
        for col, w in zip(cols, widths):
            tree.heading(col, text=col)
            tree.column(col, width=w, anchor="center")

        style = ttk.Style()
        style.configure("Treeview",
                         background=THEME["bg"],
                         foreground=THEME["text"],
                         fieldbackground=THEME["bg"],
                         font=("Consolas", 9),
                         rowheight=22)
        style.configure("Treeview.Heading",
                         background=THEME["panel2"],
                         foreground=THEME["accent"],
                         font=("Consolas", 9, "bold"))

        vsb = ttk.Scrollbar(self, orient="vertical", command=tree.yview)
        tree.configure(yscrollcommand=vsb.set)
        vsb.pack(side="right", fill="y", padx=(0, 14))
        tree.pack(fill="both", expand=True, padx=14, pady=4)

        # Bloque estático (siempre segmento 0)
        for b in proc.blocks:
            tree.insert("", "end", values=(
                "0",
                "ESTÁTICO",
                f"0x{0:08X}",
                f"0x{b.size_kb - 1:08X}",
                f"{b.start_kb}",
                f"{b.size_kb}",
            ))

        # Bloques dinámicos
        for i, b in enumerate(proc.dynamic_allocs):
            tree.insert("", "end", values=(
                f"{i + 1}",
                "DINÁMICO",
                f"0x{b.virtual_addr:08X}",
                f"0x{b.virtual_addr + b.size_kb - 1:08X}",
                f"{b.start_kb}",
                f"{b.size_kb}",
            ))

        # Nota informativa
        tk.Label(self,
                 text="Dir. Virt. Inicio: dirección relativa al proceso (base 0x00000000)\n"
                      "Dir. Real: offset en KB desde el inicio de la RAM física",
                 bg=THEME["bg"], fg=THEME["text_dim"],
                 font=("Consolas", 8),
                 justify="left").pack(anchor="w", padx=14, pady=(4, 10))


# ─────────────────────────────────────────────
#  PUNTO DE ENTRADA
# ─────────────────────────────────────────────
if __name__ == "__main__":
    app = MemOSApp()
    app.mainloop()
