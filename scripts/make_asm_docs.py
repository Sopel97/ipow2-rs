#!/usr/bin/env python3
import platform
import re
import subprocess
from pathlib import Path
import sys


ASM_DIR = Path("asm")
ASM_DIR.mkdir(exist_ok=True)


CARGO_ASM_LIST_CMD = [
    "cargo",
    "asm",
    "--bench",
    "asm",
]


# Things we don't want to dump.
SKIP_PREFIXES = (
    "core::",
    "std::",
    "alloc::",
    "asm::",
    "main",
)


WINDOWS_X86_64_TARGET = "x86_64-pc-windows-msvc"
WINDOWS_AARCH64_TARGET = "aarch64-pc-windows-msvc"
LINUX_X86_64_TARGET = "x86_64-unknown-linux-gnu"
LINUX_AARCH64_TARGET = "aarch64-unknown-linux-gnu"

CPUS_X86_64 = ["znver4", "raptorlake"]
CPUS_AARCH64 = ["apple-m1"]

ASM_FILEPATH_X86_64 = "./docs/asm-x86_64.md"
ASM_FILEPATH_AARCH64 = "./docs/asm-aarch64.md"

def make_mca_filepath(cpu):
    return f"./docs/mca-{cpu}.md"


def get_host_os():
    sysname = platform.system().lower()
    if "windows" in sysname:
        return "windows"
    if "linux" in sysname:
        return "linux"
    raise RuntimeError(f"Unsupported OS: {sysname}")


def get_target_triples(host_os: str):
    """
    Returns the most reasonable Rust target triples for:
    - x86_64 native
    - aarch64 native (if applicable ecosystem exists)
    """

    if host_os == "windows":
        return [
            WINDOWS_X86_64_TARGET,
            WINDOWS_AARCH64_TARGET,
        ]

    if host_os == "linux":
        return [
            LINUX_X86_64_TARGET,
            LINUX_AARCH64_TARGET,
        ]

    raise RuntimeError(f"No targets defined for OS: {host_os}")


def get_installed_targets():
    result = subprocess.run(
        ["rustup", "target", "list", "--installed"],
        capture_output=True,
        text=True,
        check=True,
    )
    return set(result.stdout.splitlines())


def check_targets(installed, targets):
    available = []
    missing = []

    for t in targets:
        if t in installed:
            available.append(t)
        else:
            missing.append(t)

    return available, missing


def strip_trailing_empty_lines(lines: list[str]):
    # remove trailing empty lines first
    while lines and not lines[-1].strip():
        lines.pop()


def extract_llvm_mca_legends(stdout: str):
    remaining_lines = []
    instruction_legend_lines = []
    resources_legend_lines = []

    lines_output_ptr = remaining_lines

    for line in stdout.splitlines():
        if line.startswith("Instruction Info:"):
            remaining_lines.append(line)
            lines_output_ptr = instruction_legend_lines
        elif line.startswith("Resources:"):
            remaining_lines.append(line)
            lines_output_ptr = resources_legend_lines
        else:
            curr_line_is_empty = line == "" or line.isspace()
            if curr_line_is_empty:
                lines_output_ptr = remaining_lines

            # Do not duplicate empty lines
            if not curr_line_is_empty or len(lines_output_ptr) == 0 or (lines_output_ptr[-1] != "" and not lines_output_ptr[-1].isspace()):
                lines_output_ptr.append(line)

    strip_trailing_empty_lines(remaining_lines)
    strip_trailing_empty_lines(instruction_legend_lines)
    strip_trailing_empty_lines(resources_legend_lines)

    return "\n".join(remaining_lines), "\n".join(instruction_legend_lines), "\n".join(resources_legend_lines)


def run_llvm_mca(asm: str, arch: str, cpu: str):
    # NOTE: We can only reliably do a single iteration because we can't control register allocations
    #       and it may lead to loop-carried dependencies that we don't want.
    p = subprocess.run(
        ["llvm-mca", "-x86-asm-syntax=intel", f"--march={arch}", f"-mcpu={cpu}", "-iterations=1"],
        input=asm,
        text=True,
        capture_output=True,
    )
    
    if p.returncode != 0:
        print("Command failed!")
        print("STDOUT:\n", p.stdout)
        print("STDERR:\n", p.stderr)
        raise RuntimeError("llvm-mca call failed")

    return p.stdout


def cargo_asm_list():
    return subprocess.run(
        CARGO_ASM_LIST_CMD,
        text=True,
        capture_output=True,
    ).stdout


def cargo_asm(name, target):
    cargo_asm_cmd = [
        "cargo",
        "asm",
        "--target", target,
        "--simplify",
        "--intel",
        "--bench",
        "asm",
        name,
    ]

    p = subprocess.run(
        cargo_asm_cmd,
        text=True,
        capture_output=True,
    )
    
    if p.returncode != 0:
        print("Command failed!")
        print("STDOUT:\n", p.stdout)
        print("STDERR:\n", p.stderr)
        raise RuntimeError("cargo asm call failed")
    
    return p.stdout


def list_functions():
    output = cargo_asm_list()

    functions = []

    for line in output.splitlines():
        # Matches:
        #  2 "div_floor_i128_pow2" [29]
        m = re.match(r'\s*\d+\s+"([^"]+)"', line)
        if not m:
            continue

        name = m.group(1)

        if name.startswith(SKIP_PREFIXES):
            continue

        functions.append(name)

    return functions


def get_function_asm(name, target):
    return cargo_asm(name, target)


def get_llvm_mca(asm, arch, cpu):
    return run_llvm_mca(asm, arch, cpu)


def strip_trailing_ret(s: str) -> str:
    lines = s.splitlines()

    # remove trailing empty lines first
    while lines and not lines[-1].strip():
        lines.pop()

    # now check last instruction
    if lines and lines[-1].strip().startswith("ret"):
        lines.pop()

    return "\n".join(lines)


def natural_key(s: str):
    # splits into ["div", 16, "pow", 2] style chunks
    return [
        int(part) if part.isdigit() else part
        for part in re.split(r"(\d+)", s)
    ]


def sort_function_names(names):
    return sorted(names, key=natural_key)


def choose_x86_64_target(targets):
    for target in targets:
        if "x86_64" in target:
            return target
    
def choose_aarch64_target(targets):
    for target in targets:
        if "aarch64" in target:
            return target


def produce_docs(target_x86_64, target_aarch64, cpus_x86_64, cpus_aarch64):
    functions = list_functions()
    functions = sort_function_names(functions)

    print(f"Found {len(functions)} functions")

    asms_x86_64 = dict()
    asms_aarch64 = dict()
    
    for fn in functions:
        print(f"Extracting x86_64 asm for {fn}")

        asm_x86_64 = get_function_asm(fn, target_x86_64)
        asm_x86_64 = strip_trailing_ret(asm_x86_64)

        asms_x86_64[fn] = asm_x86_64
        
        print(f"Extracting aarch64 asm for {fn}")

        asm_aarch64 = get_function_asm(fn, target_aarch64)
        asm_aarch64 = strip_trailing_ret(asm_aarch64)

        asms_aarch64[fn] = asm_aarch64

    print(f"Writing x86_64 asm to {ASM_FILEPATH_X86_64}")
    with open(ASM_FILEPATH_X86_64, "w", encoding="utf-8") as outfile:
        for fn, asm in asms_x86_64.items():
            outfile.write(f"## `{fn}`\n")
            outfile.write(f"```asm\n{asm}\n```\n")

    print(f"Writing aarch64 asm to {ASM_FILEPATH_AARCH64}")
    with open(ASM_FILEPATH_AARCH64, "w", encoding="utf-8") as outfile:
        for fn, asm in asms_aarch64.items():
            outfile.write(f"## `{fn}`\n")
            outfile.write(f"```asm\n{asm}\n```\n")

    for cpu_x86_64 in cpus_x86_64:
        with open(make_mca_filepath(cpu_x86_64), "w", encoding="utf-8") as outfile:
            instructions_legend = None
            resources_legend = None

            mcas = dict()
            for fn, asm in asms_x86_64.items():
                print(f"Extracting mca for {fn} on {cpus_x86_64}")

                mca = get_llvm_mca(asm, "x86-64", cpu_x86_64)

                mca, instructions_legend, resources_legend = extract_llvm_mca_legends(mca)

                mcas[fn] = mca

            outfile.write("# Instruction Info:\n")
            outfile.write(f"```\n{instructions_legend}\n```\n")

            outfile.write("# Resources:\n")
            outfile.write(f"```\n{resources_legend}\n```\n")
            
            outfile.write("# Functions:\n")

            for fn, mca in mcas.items():
                outfile.write(f"## `{fn}`\n")
                outfile.write(f"```asm\n{mca}\n```\n")

    for cpu_aarch64 in cpus_aarch64:
        with open(make_mca_filepath(cpu_aarch64), "w", encoding="utf-8") as outfile:
            instructions_legend = None
            resources_legend = None

            mcas = dict()
            for fn, asm in asms_aarch64.items():
                print(f"Extracting mca for {fn} on {cpus_aarch64}")

                mca = get_llvm_mca(asm, "aarch64", cpu_aarch64)

                mca, instructions_legend, resources_legend = extract_llvm_mca_legends(mca)

                mcas[fn] = mca

            outfile.write("# Instruction Info:\n")
            outfile.write(f"```\n{instructions_legend}\n```\n")

            outfile.write("# Resources:\n")
            outfile.write(f"```\n{resources_legend}\n```\n")
            
            outfile.write("# Functions:\n")

            for fn, mca in mcas.items():
                outfile.write(f"## `{fn}`\n")
                outfile.write(f"```asm\n{mca}\n```\n")


def main():
    host_os = get_host_os()
    print(f"Detected OS: {host_os}")

    targets = get_target_triples(host_os)
    print(f"Expected targets: {targets}")

    try:
        installed = get_installed_targets()
    except FileNotFoundError:
        print("rustup not found in PATH")
        sys.exit(1)

    available, missing = check_targets(installed, targets)

    print("\nInstalled targets:")
    for t in installed:
        print(f"  - {t}")

    print("\nResults:")
    print("Available:")
    for t in available:
        print(f"  - {t}")

    print("Missing:")
    for t in missing:
        print(f"  - {t}")

    if missing:
        print("\nTo install missing targets:")
        for t in missing:
            print(f"  rustup target add {t}")

    if missing:
        return
    
    target_x86_64 = choose_x86_64_target(available)
    if target_x86_64 is None:
        print("Unexpectedly missing an x86_64 target")
        return
    
    target_aarch64 = choose_aarch64_target(available)
    if target_aarch64 is None:
        print("Unexpectedly missing an aarch64 target")
        return
    
    produce_docs(
        target_x86_64=target_x86_64,
        target_aarch64=target_aarch64,
        cpus_x86_64=CPUS_X86_64,
        cpus_aarch64=CPUS_AARCH64,
    )


if __name__ == "__main__":
    main()