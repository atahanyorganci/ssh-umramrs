# Bilkent University UMRAM Machine Checker

The Bilkent University is home to [National Magnetic Resonance Research Center (UMRAM)](https://umram.bilkent.edu.tr/). Many graduate and undergraduate students from engineering to neuroscience departments conduct their research using the machines in the center. These machines are equipped with powerful CPUs and GPUs. However, organization of the machines and their IP addresses change frequently without a notice. This program sweeps the subdomain of the center and finds all the machines your credentials allow you to connect and list their GPUs.

## Usage

> [!NOTE]
> In order to connect these machines you need to have valid credentials, and you need to be connected to Bilkent network (either physically or via VPN).

> [!WARNING]
> Currently, you need to update the [source code with your credentials](https://github.com/tunakasif/ssh-umramrs/blob/9a26b4e83264e85f0aec667576837a1fc3f45f5a/src/main.rs#L12-L13). This is not ideal and will be updated in the future.

The program asynchronously tries to ssh into the machines with given IP addresses at their port 22, and get GPU configuration through `nvidia-smi` command (only NVIDIA GPUs are available on these machines). The output is in the format of `IP:Port` followed by the list of GPUs. The program can be run with the following command.

```sh
cargo run --release > state.log
```

An example usage is shown below with piping the output to a file.

![Demo GIF](./.github/assets/demo.gif)

A sample `state.log` file is provided below (UUID information redacted for privacy). Note that this is a snapshot of the machines at a certain time, and the machines may change.

```stdout
139.179.121.52:22:
    GPU 0: NVIDIA GeForce RTX 4090
    GPU 1: NVIDIA GeForce RTX 4090
139.179.121.68:22:
    GPU 0: TITAN Xp
139.179.121.71:22:
    GPU 0: GeForce GTX 1080 Ti
139.179.121.72:22:
    GPU 0: NVIDIA GeForce RTX 2080 Ti
139.179.121.73:22:
    GPU 0: NVIDIA GeForce RTX 2080 Ti
    GPU 1: NVIDIA GeForce RTX 2080 Ti
139.179.121.74:22:
    GPU 0: NVIDIA GeForce GTX 1080 Ti
    GPU 1: NVIDIA GeForce GTX 1080 Ti
139.179.121.77:22:
    GPU 0: NVIDIA RTX A4000
    GPU 1: NVIDIA RTX A4000
    GPU 2: NVIDIA RTX A4000
    GPU 3: NVIDIA RTX A4000
139.179.121.79:22:
    GPU 0: NVIDIA GeForce RTX 4090
    GPU 1: NVIDIA GeForce RTX 4090
```
