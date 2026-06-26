import subprocess
import cpuinfo
import os


BENCH_NAME = "public"

DEFAULT_BENCH_FILEPATH = "./docs/bench-default.md"
NATIVE_BENCH_FILEPATH = "./docs/bench-native.md"


def get_cpu_name() -> str:
    return cpuinfo.get_cpu_info()['brand_raw']


def run_bench(name: str, target_cpu: str | None):
    env = os.environ.copy()
    if target_cpu:
        env["RUSTFLAGS"] = f"-C opt-level=3 -C target-cpu={target_cpu}"
    else:
        env["RUSTFLAGS"] = "-C opt-level=3"

    output = []
    do_capture = False

    with subprocess.Popen(
        ["cargo", "bench", name],
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        bufsize=1,
        env=env,
        encoding="utf-8",
    ) as p:
        assert p.stdout is not None

        for line in p.stdout:
            print(line, end="", flush=True)

            # Omit compilation and info. First line is something like the following
            # public       fastest       │ slowest       │ median        │ mean          │ samples │ iters
            if line.startswith(name):
                do_capture = True

            if do_capture:
                output.append(line)

        ret = p.wait()

    if ret != 0:
        raise RuntimeError("cargo bench failed")

    return "".join(output)


def main() -> None:
    bench_default = run_bench(BENCH_NAME, None)
    bench_native = run_bench(BENCH_NAME, "native")

    with open(DEFAULT_BENCH_FILEPATH, "w", encoding="utf-8") as outfile:
        outfile.write(f"# `{BENCH_NAME}` bench with default target\n")
        outfile.write(f"```\n{bench_default}\n```\n")

    with open(NATIVE_BENCH_FILEPATH, "w", encoding="utf-8") as outfile:
        cpu = cpuinfo.get_cpu_info()['brand_raw']
        outfile.write(f"# `{BENCH_NAME}` bench with native target ({cpu})\n")
        outfile.write(f"```\n{bench_native}\n```\n")


if __name__ == "__main__":
    main()