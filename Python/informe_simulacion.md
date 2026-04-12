# Informe Técnico: Simulación de Scheduler de Procesos (SimuOS)

Este informe detalla el diseño, lógica y construcción del simulador de planificación de procesos y gestión de recursos de red desarrollado en Python.

---

## 1. Estructura de la Tabla de Procesos
Para la gestión del sistema, se implementó una estructura de datos centralizada donde cada proceso se identifica de forma única y posee los siguientes atributos:

*   **PID (Process ID)**: Un identificador numérico único (comenzando desde 1 para facilitar el seguimiento).
*   **Estado**: El proceso transita por cinco estados fundamentales: `Creación`, `Corriendo`, `Suspendido`, `Detenido` y `Finalizado`.
*   **Propietario (Owner)**: Se definieron cuatro usuarios del sistema para simular un entorno multiusuario: `root`, `admin`, `estudiante` y `usuario`.
*   **Recurso de Red**: Cada proceso tiene asignado un puerto TCP (en el rango **80 al 85**) el cual debe disputar para poder ejecutarse.

---

## 2. Lógica del Scheduler y Competencia de Recursos
El "corazón" del sistema es el algoritmo de **Exclusión Mutua** para el uso de puertos TCP:

1.  **Validación de Estado**: Al intentar pasar al estado **"Corriendo"**, el scheduler verifica si el puerto TCP asignado al proceso ya está siendo utilizado por otro proceso activo.
2.  **Manejo de Conflictos**:
    *   Si el recurso está ocupado, el proceso es movido automáticamente al estado **"Suspendido"**.
    *   El PID del proceso entra en una **Lista de Espera (FIFO)** asociada específicamente a ese puerto.
3.  **Liberación y Activación**: Cuando el proceso que ocupa el puerto termina o es detenido, el scheduler consulta la lista de espera del recurso. El primer proceso en la cola es activado y pasa al estado **"Corriendo"** de forma inmediata, garantizando equidad y orden.

---

## 3. Presentación Gráfica y Monitor de Recursos
La interfaz visual proporciona una vista en tiempo real del estado del hardware simulado:

*   **Monitor de Puertos TCP**: Se presenta una barra de indicadores para los puertos 80 al 85.
*   **Indicadores de Color**: 
    *   **Verde**: Indica que el puerto está libre.
    *   **Rojo**: Indica que el puerto está ocupado (muestra el PID del dueño).
*   **Visualización de Colas**: Debajo de cada puerto se muestra dinámicamente la lista de PIDs que están esperando por ese recurso específico.

---

## 4. Ventana de Comandos (Terminal Simulator)
Se integró una consola interactiva que permite al usuario actuar como administrador del sistema mediante los siguientes comandos:

*   `ps`: Lista todos los procesos activos, mostrando su PID, propietario y estado actual.
*   `ps -u [username]`: Filtra la lista anterior para mostrar solo los procesos pertenecientes al usuario especificado (ej. `ps -u root`).
*   `kill [pid]`: Envía una señal de terminación al proceso, liberando sus recursos y moviéndolo al estado finalizado.
*   `stress`: Comando adicional para inyectar 20 procesos y poner a prueba la lógica de colas del sistema.

---

## 5. Proceso de Construcción y Uso de IA
La construcción del código se realizó de forma iterativa utilizando **Inteligencia Artificial** como compañero de programación (Pair Programming):

1.  **Definición de Requerimientos**: Se tradujeron las reglas de negocio (puertos TCP, colas FIFO) a una estructura de clases en Python.
2.  **Refactorización Estética**: Se utilizó la IA para transformar una interfaz básica en un panel de control funcional, asegurando que los elementos críticos (como los botones de inicio y la terminal) fueran intuitivos.
3.  **Depuración de Lógica**: La IA ayudó a implementar la recursividad necesaria para que, al liberar un recurso, el sistema "despertara" automáticamente al siguiente proceso en la cola sin intervención del usuario.

---

## 6. Comparación con un Scheduler Comercial
Aunque el simulador es funcional, existen diferencias clave con un kernel de la vida real (como el de Windows o Linux):

*   **Prioridades**: Los SO reales no solo usan orden de llegada (FIFO), sino niveles de prioridad (Tiempo Real vs Background).
*   **Preemption (Quantum)**: En la realidad, el SO quita la CPU a un proceso tras unos milisegundos para dársela a otro. Aquí, los cambios son aleatorios o manuales.
*   **Gestión de Memoria**: Un scheduler real debe coordinarse con la RAM y el archivo de paginación, algo que en este modelo se simplifica al enfocarse solo en puertos de red.
*   **Multicore**: El simulador maneja un flujo único, mientras que un SO moderno debe balancear procesos entre múltiples núcleos físicos del procesador.
