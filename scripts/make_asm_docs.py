#!/usr/bin/env python3
import platform
import re
import subprocess
from pathlib import Path
import sys


ASM_DIR = Path("asm")
ASM_DIR.mkdir(exist_ok=True)


CARGO_ASM_LIST_CMD: list[str] = [
    "cargo",
    "asm",
    "--bench",
    "asm",
]


# Things we don't want to dump.
SKIP_PREFIXES: tuple[str, ...] = (
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

CPUS_X86_64: list[str] = ["znver4", "raptorlake"]
CPUS_AARCH64: list[str] = ["apple-m1"]

ASM_FILEPATH_X86_64 = "./docs/asm-x86_64.md"
ASM_FILEPATH_AARCH64 = "./docs/asm-aarch64.md"


def make_mca_filepath(cpu: str) -> str:
    return f"./docs/mca-{cpu}.md"


def get_host_os() -> str:
    sysname = platform.system().lower()
    if "windows" in sysname:
        return "windows"
    if "linux" in sysname:
        return "linux"
    raise RuntimeError(f"Unsupported OS: {sysname}")


def get_target_triples(host_os: str) -> list[str]:
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


def get_installed_targets() -> set[str]:
    result = subprocess.run(
        ["rustup", "target", "list", "--installed"],
        capture_output=True,
        text=True,
        check=True,
    )
    return set(result.stdout.splitlines())


def check_targets(
    installed: set[str],
    targets: list[str],
) -> tuple[list[str], list[str]]:
    available: list[str] = []
    missing: list[str] = []

    for t in targets:
        if t in installed:
            available.append(t)
        else:
            missing.append(t)

    return available, missing


def strip_trailing_empty_lines(lines: list[str]) -> None:
    while lines and not lines[-1].strip():
        lines.pop()


def extract_llvm_mca_legends(
    stdout: str,
) -> tuple[str, str, str]:
    remaining_lines: list[str] = []
    instruction_legend_lines: list[str] = []
    resources_legend_lines: list[str] = []

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
            if not curr_line_is_empty or len(lines_output_ptr) == 0 or (
                lines_output_ptr[-1] != "" and not lines_output_ptr[-1].isspace()
            ):
                lines_output_ptr.append(line)

    strip_trailing_empty_lines(remaining_lines)
    strip_trailing_empty_lines(instruction_legend_lines)
    strip_trailing_empty_lines(resources_legend_lines)

    return (
        "\n".join(remaining_lines),
        "\n".join(instruction_legend_lines),
        "\n".join(resources_legend_lines),
    )


def run_llvm_mca(asm: str, arch: str, cpu: str) -> str:
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


def cargo_asm_list() -> str:
    return subprocess.run(
        CARGO_ASM_LIST_CMD,
        text=True,
        capture_output=True,
    ).stdout


def cargo_asm(number: str, name: str, target: str) -> str:
    cargo_asm_cmd = [
        "cargo",
        "asm",
        "--target", target,
        "--simplify",
        "--intel",
        "--bench",
        "asm",
        number,
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


def list_functions() -> list[tuple[str, str]]:
    output = cargo_asm_list()

    functions: list[tuple[str, str]] = []

    for line in output.splitlines():
        # Matches:
        #  2 "div_floor_i128_pow2" [29]
        m = re.match(r'\s*(\d+)\s+"([^"]+)"', line)
        if not m:
            continue

        number = m.group(1)
        name = m.group(2)

        if name.startswith(SKIP_PREFIXES):
            continue

        functions.append((number, name))

    return functions


def get_function_asm(number: str, name: str, target: str) -> str:
    return cargo_asm(number, name, target)


def get_llvm_mca(asm: str, arch: str, cpu: str) -> str:
    return run_llvm_mca(asm, arch, cpu)


def strip_trailing_ret(s: str) -> str:
    lines = s.splitlines()

    while lines and not lines[-1].strip():
        lines.pop()

    if lines and lines[-1].strip().startswith("ret"):
        lines.pop()

    return "\n".join(lines)


def natural_key(s: str) -> list[int | str]:
    return [
        int(part) if part.isdigit() else part
        for part in re.split(r"(\d+)", s)
    ]


def sort_functions(functions: list[tuple[str, str]]) -> list[tuple[str, str]]:
    return sorted(functions, key=lambda x: natural_key(x[1]))


def choose_x86_64_target(targets: list[str]) -> str | None:
    for target in targets:
        if "x86_64" in target:
            return target
    return None


def choose_aarch64_target(targets: list[str]) -> str | None:
    for target in targets:
        if "aarch64" in target:
            return target
    return None


def produce_docs(
    target_x86_64: str,
    target_aarch64: str,
    cpus_x86_64: list[str],
    cpus_aarch64: list[str],
) -> None:
    functions = list_functions()
    functions = sort_functions(functions)

    print(f"Found {len(functions)} functions")

    jobs: list[tuple[str, str, list[str], str]] = [
        # march    target         cpus         asm filepath
        ("x86-64",  target_x86_64,  cpus_x86_64,  ASM_FILEPATH_X86_64),
        ("aarch64", target_aarch64, cpus_aarch64, ASM_FILEPATH_AARCH64),
    ]

    for (march, target, cpus, asm_filepath) in jobs:
        asms: dict[str, str] = {}

        for (number, fn) in functions:
            print(f"Extracting {march} asm for {fn}")

            asm = get_function_asm(number, fn, target)
            asm = strip_trailing_ret(asm)

            asms[fn] = asm

        print(f"Writing {march} asm to {asm_filepath}")
        with open(asm_filepath, "w", encoding="utf-8") as outfile:
            for fn, asm in asms.items():
                outfile.write(f"## `{fn}`\n")
                outfile.write(f"```asm\n{asm}\n```\n")

        for cpu in cpus:
            with open(make_mca_filepath(cpu), "w", encoding="utf-8") as outfile:
                instructions_legend: str = ""
                resources_legend: str = ""

                mcas: dict[str, str] = {}
                for fn, asm in asms.items():
                    print(f"Extracting mca for {fn} on {cpu}")

                    mca = get_llvm_mca(asm, march, cpu)

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


def main() -> None:
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