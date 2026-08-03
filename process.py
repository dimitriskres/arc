import sys
import os
import contextlib
import subprocess
import shutil
import tempfile
import csv
import re
import decimal
import tqdm
import collections
import statistics
import copy
import tabulate
import matplotlib.pyplot as plt

NAME = sys.argv[1]

SOLVERS = ['gecode', 'chuffed', 'cp-sat']

TICKS = list(map(str, range(1, 11)))

ROOT_PATH = '.'

BASE_PATH = os.path.join(ROOT_PATH, 'puzzles', NAME)

DATA_PATH = os.path.join(BASE_PATH, 'media')

os.makedirs(DATA_PATH, exist_ok = True)

EXEC_PATH = os.path.join(ROOT_PATH, 'target', 'release')

ZINC_PATH = os.path.join(BASE_PATH, 'zinc')

ZINC_MZN_PATH = os.path.join(ZINC_PATH, 'model.mzn')

TUNE_UARCH_PATH_BASE = os.path.join(BASE_PATH, 'vtune')

KERNEL_SOLVE_VERSIONS = {
    'chain': ['0']
}

KERNEL_SOLVE_VERSIONS = KERNEL_SOLVE_VERSIONS.get(NAME, ['0', '1'])

KERNEL_UARCH_PS1_PATH = os.path.join(ROOT_PATH, 'kernel-uarch.ps1')

KERNEL_BENCH_CSV_PATH = os.path.join(BASE_PATH, 'kernel-bench.csv')

SOLVER_UARCH_PS1_PATH = os.path.join(ROOT_PATH, 'solver-uarch.ps1')

GENERAL_UARCH_TIMEOUT = 60

SOLVER_BENCH_CSV_PATH = os.path.join(BASE_PATH, 'solver-bench.csv')

SOLVER_BENCH_TIMEOUT = 60

SOLVER_BENCH_COUNT = 5

SOLVER_BENCH_SETUP_TIME_PATTERN = re.compile(r'%%%mzn-stat:\s*initTime=([0-9.]+)')

SOLVER_BENCH_SOLVE_TIME_PATTERN = re.compile(r'%%%mzn-stat:\s*solveTime=([0-9.]+)')

TUNE_DISPLAY_METRICS = [
    [
        ('Elapsed Time', 0, 's'),
        ('Clockticks', 0, 'i'),
        ('Instructions Retired', 0, 'i'),
        ('CPI Rate', 0, 'd%'),
        ('MUX Reliability', 0, 'p%'),
    ],
    [
        ('Retiring', 0, 'd%'),
    ],
    [
        ('Front-End Bound', 0, 'd%'),
        ('Front-End Latency', 1, 'd%'),
        ('Front-End Bandwidth', 1, 'd%'),
    ],
    [
        ('Bad Speculation', 0, 'd%'),
        ('Branch Mispredict', 1, 'd%'),
    ],
    [
        ('Back-End Bound', 0, 'd%'),
        ('Memory Bound', 1, 'd%'),
        ('L1 Bound', 2, 'd%'),
        ('L2 Bound', 2, 'd%'),
        ('L3 Bound', 2, 'd%'),
        ('DRAM Bound', 2, 'd%'),
        ('Store Bound', 2, 'd%'),
        ('Core Bound', 1, 'd%'),
        ('Divider', 2, 'd%'),
        ('Serializing Operations', 2, 'd%'),
        ('Port Utilization', 2, 'd%')
    ]
]

ROOT_TEMPLATE_PATH = os.path.join(ROOT_PATH, 'template.md')

BASE_TEMPLATE_PATH = os.path.join(BASE_PATH, 'template.md')

BASE_README_PATH = os.path.join(BASE_PATH, 'README.md')

def run(command: list[str], capture: bool = False, stdout = None, stderr = None, text: bool = True, timeout: int | None = None, codes = [0]):
    
    mode = 'w+' if text else 'w+b'
    
    with contextlib.ExitStack() as stack:
        
        if capture:
            stdout = stack.enter_context(tempfile.TemporaryFile(mode = mode))
            stderr = stack.enter_context(tempfile.TemporaryFile(mode = mode))

        process = subprocess.Popen(command, cwd = ROOT_PATH, stdout = stdout, stderr = stderr, text = text, creationflags = subprocess.CREATE_NEW_PROCESS_GROUP)

        expired = False
        
        try:
            process.wait(timeout = timeout)
        except subprocess.TimeoutExpired:
            expired = True
            subprocess.run(['taskkill', '/PID', str(process.pid), '/T', '/F'], stdout = subprocess.DEVNULL, stderr = subprocess.DEVNULL, check = False)
            process.wait()

        if capture:
            stdout.seek(0)
            output = stdout.read()
            stderr.seek(0)
            errors = stderr.read()
        else:
            output = None
            errors = None

        if expired:
            raise subprocess.TimeoutExpired(command, timeout, output = output, stderr = errors)

        if not process.returncode in codes:
            raise subprocess.CalledProcessError(process.returncode, command, output = output, stderr = errors)

        return subprocess.CompletedProcess(command, process.returncode, output, errors)

def get_relative_path(path: str) -> str:
    
    return os.path.relpath(path, start = BASE_PATH)

def get_rust_bin_name(name: str):
    
    return f'{NAME}-{name}'

def get_rust_exe_path(name: str):
    
    return os.path.join(EXEC_PATH, get_rust_bin_name(name) + '.exe')

def get_zinc_dzn_name(tick: str):
    
    return f'data-{tick}.dzn'    

def get_zinc_dzn_path(tick: str):
    
    return os.path.join(ZINC_PATH, get_zinc_dzn_name(tick))

def get_zinc_fzn_name(name: str, tick: str):
    
    return f'model-{name}-{tick}.fzn'

def get_zinc_fzn_path(name: str, tick: str):
    
    return os.path.join(ZINC_PATH, get_zinc_fzn_name(name, tick))

def get_tune_out_path(name: str):
    
    return os.path.join(TUNE_UARCH_PATH_BASE, name)

def get_data_path(name: str):
    
    return os.path.join(DATA_PATH, name)

def rust_compile(name: str, **kwargs):
    
    name = get_rust_bin_name(name)
    
    command = ['cargo', 'build', '--bin', name, '--release']
    
    return run(command, **kwargs)

def rust_execute(name: str, *args, **kwargs):
    
    name = get_rust_exe_path(name)
    
    command = [name, *args]

    return run(command, **kwargs)

def zinc_compile(name: str, tick: str):
    
    dzn_path = get_zinc_dzn_path(tick)
    
    rust_execute('stage', '--tick', tick, '--path', dzn_path)
    
    fzn_path = get_zinc_fzn_path(name, tick)
    
    command = ['MINIZINC', '-c', '--solver', name, ZINC_MZN_PATH, dzn_path, '--fzn', fzn_path, '--no-output-ozn']
    
    run(command)
    
def zinc_execute(name: str, tick: str, **kwargs):
    
    fzn_path = get_zinc_fzn_path(name, tick)
    
    command = ['MINIZINC', '--solver', name, fzn_path, '--statistics']
    
    return run(command, **kwargs)

def tune_uarch(path: str, command: list[str]):

    if os.path.exists(path):
        shutil.rmtree(path)
        
    command = [
        'vtune', 
        '-collect', 'uarch-exploration', 
        '-no-summary', 
        '-duration', str(GENERAL_UARCH_TIMEOUT),
        '-result-dir', 
        path, 
        '--', 
        'cmd.exe', 
        '/c', 'start', '', 
        '/b', 
        '/wait', 
        '/affinity', 'FFF', 
        *command
    ]
        
    run(command, codes = [0, 4])
    

def tune_export(name: str) -> list[dict[str, str]]:
    
    path = get_tune_out_path(name)
    
    with tempfile.NamedTemporaryFile(mode = 'w+', encoding = 'utf-8', newline = '', suffix = '.csv', delete = True) as file:
        
        command = [
            'vtune', 
            '-report', 'summary', 
            '-format', 'csv',
            '-csv-delimiter', 'comma',
            '-report-knob show-issues=false',
            '-result-dir',
            path
        ]
    
        run(command, capture = False, stdout = file)
        
        file.seek(0)
        
        table = list(csv.DictReader(file))
        
    return table

def tune_display(table: list[dict[str, str]]) -> list[tuple[str, str]]:
    
    ignore = None
    
    statistics = {}
    
    for record in table:
        name = record['Metric Name']
        data = record['Metric Value']
        level = int(record['Hierarchy Level'])
        if not ignore is None:
            if level > ignore:
                continue
            ignore = None
        if name == 'Efficient-core (E-core)':
            ignore = level
            continue
        if data == '':
            continue
        statistics[name] = data

    name_size = 0
    data_size = 0
    
    for group in TUNE_DISPLAY_METRICS:
        for name, level, kind in group:
            data = statistics[name]
            match kind:
                case 's':
                    data = f'{float(data):.2f}s'
                case 'i':
                    data = f'{int(data):,}'
                case 'd%':
                    data = f'{float(data):.1f}%'
                case 'p%':
                    data = f'{100 * float(data):.1f}%'
            name_size = max(name_size, len(name) + level * 2)
            data_size = max(data_size, len(data))
            statistics[name] = data
    
    buffer: list[str] = []

    for group in TUNE_DISPLAY_METRICS:
        for name, level, _ in group:
            data = statistics[name]
            name = '  ' * level + name
            buffer.append(f'{name.ljust(name_size)}  {data.rjust(data_size)}')
        buffer.append('')

    return '\n'.join(buffer)

def kernel_uarch(version: str):
    
    out_path = get_tune_out_path(f'kernel-{version}')
    
    bin_path = get_rust_exe_path('solve')

    command = [
        'powershell.exe', 
        '-NoProfile', 
        '-ExecutionPolicy', 'Bypass', 
        '-File', KERNEL_UARCH_PS1_PATH, 
        '-Path', bin_path,
        '-Version', version
    ]
        
    tune_uarch(out_path, command)
    
def solver_uarch(name: str):
    
    tick = TICKS[- 1]

    fzn_path = get_zinc_fzn_path(name, tick)
    
    out_path = get_tune_out_path(name)
    
    command = [
        'powershell.exe', 
        '-NoProfile', 
        '-ExecutionPolicy', 'Bypass', 
        '-File', SOLVER_UARCH_PS1_PATH, 
        '-Name', name, 
        '-Path', fzn_path
    ]
        
    tune_uarch(out_path, command)

def kernel_bench(writer, version: str):
    
    bin_path = get_rust_exe_path('solve')
    
    with tqdm.tqdm(total = len(TICKS) * SOLVER_BENCH_COUNT) as progress:
        
        for tick in TICKS:
            
            progress.desc = f'kernel [v:{version} t:{tick}]'

            for _ in range(SOLVER_BENCH_COUNT):
                
                command = [bin_path, '--tick', tick, '--version', version]
                    
                try:
                    process = run(command, timeout = SOLVER_BENCH_TIMEOUT, capture = True)
                except subprocess.TimeoutExpired:
                    return
                
                solve_duration, total_duration = process.stdout.split(',')
                
                solve_duration = int(solve_duration)
                
                total_duration = int(total_duration)
                
                setup_duration = total_duration - solve_duration
                
                writer.writerow([tick, 'kernel', version, setup_duration, solve_duration])
                
                progress.update(1)
            
def solver_bench(writer, name: str):
    
    with tqdm.tqdm(total = len(TICKS) * SOLVER_BENCH_COUNT) as progress:
        
        for tick in TICKS:
            
            progress.desc = f'{name} [v:- t:{tick}]'

            for _ in range(SOLVER_BENCH_COUNT):
                
                try:
                    process = zinc_execute(name, tick, timeout = SOLVER_BENCH_TIMEOUT, capture = True)
                except subprocess.TimeoutExpired:
                    return
                
                output = process.stdout
                
                setup_match = SOLVER_BENCH_SETUP_TIME_PATTERN.search(output)

                if setup_match is None:
                    setup_duration = 0
                else:
                    setup_duration = int(decimal.Decimal(setup_match.group(1)) * 1_000_000_000)
                    
                solve_match = SOLVER_BENCH_SOLVE_TIME_PATTERN.search(output)

                if solve_match is None:
                    solve_duration = 0
                else:
                    solve_duration = int(decimal.Decimal(solve_match.group(1)) * 1_000_000_000)
                
                writer.writerow([tick, name, '-', setup_duration, solve_duration])
                
                progress.update(1)
                
def kernel_bench_display():
    
    records = collections.defaultdict(list)
    with open(KERNEL_BENCH_CSV_PATH, 'r', newline = '') as file:
        for (name, duration) in csv.reader(file):
            records[name].append(int(duration))
            
    rows = []
    for name, durations in records.items():
        version, type, name = name.split('_', 2)
        version = version.lower().removeprefix('v')
        modules = (part.ljust(3, ' ') for part in name.split('_') if not part.startswith('P'))
        name = ' '.join(modules)
        mean = statistics.mean(durations)
        stdv = statistics.stdev(durations)
        stdv = stdv / mean * 100
        mean = mean / 1_000_000
        rows.append([type, name, version, mean, stdv])
        
    rows = sorted(rows, key = lambda row: (row[3], row[4]))

    tabrows = []
        
    for row in rows:
        row = copy.deepcopy(row)
        for col in [0, 1, 2]:
            row[col] = f'`{row[col]}`'
        tabrows.append(row)
        
    text = tabulate.tabulate(
        tabrows, 
        headers = ['type', 'name', 'v', 'mean', 'stdv'], 
        tablefmt = 'pipe', 
        floatfmt = ['', '', '', '.0f', '.1f'],
        colalign = ['left', 'left', 'left', 'right', 'right']
    )
    
    return (rows, text)

def solver_bench_display():

    records = collections.defaultdict(list)
    with open(SOLVER_BENCH_CSV_PATH, 'r', newline = '') as file:
        for (tick, name, version, setup_duration, solve_duration) in csv.reader(file):
            records[(int(tick), name, version)].append((int(setup_duration), int(solve_duration)))
            
    rows = []
    for (tick, name, version), durations in records.items():
        setup_durations, solve_durations = zip(*durations)
        setup_mean = statistics.mean(setup_durations)
        setup_stdv = statistics.stdev(setup_durations)
        solve_mean = statistics.mean(solve_durations)
        solve_stdv = statistics.stdev(solve_durations)
        setup_stdv = (setup_stdv / setup_mean * 100) if setup_mean != 0 else 0
        solve_stdv = (solve_stdv / solve_mean * 100) if solve_mean != 0 else 0
        setup_mean = setup_mean / 1_000_000
        solve_mean = solve_mean / 1_000_000
        rows.append([tick, name, version, setup_mean, setup_stdv, solve_mean, solve_stdv])
        
    rows = sorted(rows, key = lambda row: (- row[0], row[3] + row[5], row[6], row[4]))
    
    tabrows = []
    
    for row in rows:
        row = copy.deepcopy(row)
        for col in [0, 1, 2]:
            row[col] = f'`{row[col]}`'
        tabrows.append(row)
    
    text = tabulate.tabulate(
        tabrows, 
        headers = ['tick', 'name', 'v', 'st mean', 'st stdv', 'sv mean', 'sv stdv'], 
        tablefmt = 'pipe', 
        floatfmt = ['', '', '', '.0f', '.1f', '.0f', '.1f'],
        colalign = ['left', 'left', 'left', 'right', 'right', 'right', 'right']
    )
    
    return (rows, text)

def solver_bench_plot():
    
    records = collections.defaultdict(list)

    with open(SOLVER_BENCH_CSV_PATH, 'r', newline = '') as file:
        for tick, name, version, setup_duration, solve_duration in csv.reader(file):
            total_duration = int(setup_duration) + int(solve_duration)
            records[(name, version, int(tick))].append(total_duration)

    series = collections.defaultdict(dict)

    for (name, version, tick), durations in records.items():
        total_mean = statistics.mean(durations) / 1_000_000
        series[(name, version)][tick] = total_mean

    last_tick = max(tick for tick_values in series.values() for tick in tick_values)
    last_tick_values = [tick_values[last_tick] for tick_values in series.values() if last_tick in tick_values]
    last_tick_average = statistics.mean(last_tick_values)

    values_below_average = [value for tick_values in series.values() for value in tick_values.values() if value < last_tick_average]
    max_below_average = max(values_below_average)

    first_tick_values = [tick_values[1] for tick_values in series.values() if 1 in tick_values]
    max_first_tick = max(first_tick_values)

    y_max = max_below_average + max_first_tick

    fig, ax = plt.subplots(figsize = (12, 7))

    for (name, version), tick_values in sorted(series.items()):
        ticks = sorted(tick_values)
        means = [tick_values[tick] for tick in ticks]
        ax.plot(ticks, means, marker = 'o', label = f'{name} ({version})')

    ax.set_title('Average Total Solver Time by Tick')
    ax.set_xlabel('Tick')
    ax.set_ylabel('Average setup + solve time (ms)')
    ax.set_xticks(range(1, 11))
    ax.set_ylim(0, y_max)
    ax.grid(True, alpha = 0.3)
    ax.legend(title = 'Solver', bbox_to_anchor = (1.02, 1), loc = 'upper left')

    fig.tight_layout()
    
    path = get_data_path(f'solver-bench-plot.svg')
    
    fig.savefig(path, format = 'svg', bbox_inches = 'tight')
    
    return path

def main_perform():
    
    for name in ['stage', 'solve']:
        rust_compile(name)
    
    for name in SOLVERS:
        for tick in TICKS:
            zinc_compile(name, tick)
    
    if os.path.exists(SOLVER_BENCH_CSV_PATH):
            os.remove(SOLVER_BENCH_CSV_PATH)
        
    with open(SOLVER_BENCH_CSV_PATH, 'a', newline = '') as file:

        writer = csv.writer(file)

        for version in KERNEL_SOLVE_VERSIONS:
            kernel_bench(writer, version)
        
        for name in SOLVERS:
            solver_bench(writer, name)
    
    for version in KERNEL_SOLVE_VERSIONS:
        kernel_uarch(version)
    
    for name in SOLVERS:
        solver_uarch(name)

def main_display():
    
    kernel_bench_csv_path = KERNEL_BENCH_CSV_PATH
    
    solver_bench_csv_path = SOLVER_BENCH_CSV_PATH
    
    kernel_bench_rows, kernel_bench_text = kernel_bench_display()
        
    solver_bench_rows, solver_bench_text = solver_bench_display()
    
    fast_key = lambda row: (row[3], row[4])

    fast_scalar_A_row = min((row for row in kernel_bench_rows if row[0] == 'scalar'), key = fast_key)
    fast_object_X_row = min((row for row in kernel_bench_rows if row[0] == 'object' and 'X' in row[1]), key = fast_key)
    fast_object_R_row = min((row for row in kernel_bench_rows if row[0] == 'object' and 'R' in row[1]), key = fast_key)
    
    fast_scalar_A_v = fast_scalar_A_row[2]
    fast_scalar_A_name = fast_scalar_A_row[1]
    fast_scalar_A_time = round(fast_scalar_A_row[3])
    
    fast_object_X_v = fast_object_X_row[2]
    fast_object_X_name = fast_object_X_row[1]
    fast_object_X_time = round( fast_object_X_row[3])
    
    fast_object_R_v = fast_object_R_row[2]
    fast_object_R_name = fast_object_R_row[1]
    fast_object_R_time = round( fast_object_R_row[3])

    solver_bench_svg_path = solver_bench_plot()
    
    tune_display_names = []
    
    for version in KERNEL_SOLVE_VERSIONS:
        tune_display_names.append(f'kernel-{version}')
        
    for name in SOLVERS:
        tune_display_names.append(name)

    tune_display_texts = []
    
    for name in tune_display_names:
        text = tune_display(tune_export(name))
        tune_display_texts.append(f'### {name}\n\n```\n{text}\n```')
        
    tune_display_text = '\n\n'.join(tune_display_texts)

    with open(BASE_TEMPLATE_PATH, mode = 'r') as file:
        base_template = file.read()
        
    with open(ROOT_TEMPLATE_PATH, mode = 'r') as file:
        root_template = file.read()
    
    text = root_template.format(
        fast_scalar_A_v = fast_scalar_A_v,
        fast_scalar_A_name = fast_scalar_A_name,
        fast_scalar_A_time = fast_scalar_A_time,
        fast_object_X_v = fast_object_X_v,
        fast_object_X_name = fast_object_X_name,
        fast_object_X_time = fast_object_X_time,
        fast_object_R_v = fast_object_R_v,
        fast_object_R_name = fast_object_R_name,
        fast_object_R_time = fast_object_R_time,
        kernel_bench_csv_path = get_relative_path(kernel_bench_csv_path),
        solver_bench_mzn_path = get_relative_path(ZINC_MZN_PATH),
        solver_bench_csv_path = get_relative_path(solver_bench_csv_path),
        solver_bench_svg_path = get_relative_path(solver_bench_svg_path),
        solver_bench_text = solver_bench_text,
        kernel_bench_text = kernel_bench_text,
        tune_display_text = tune_display_text
    )
    
    text = base_template.format(
        process_text = text
    )
    
    with open(BASE_README_PATH, mode = 'w') as file:
        file.write(text)
    
if __name__ == '__main__':
    # main_perform()
    main_display()
