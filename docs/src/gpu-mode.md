# GPU Mode ⚡ {#gpu-mode}

::: danger Why is this section important?
Screenshots can take **A LOT** of time to complete.
GPU Mode can lower the computation duration from `20 hours` to `20 seconds`.
Consequently it is greatly recommended to make sure GPU Mode is enabled before starting a screenshot.
:::

GPU Mode allows you to make use of the parallel processing capabilities of your hardware, which enormously **improves the rendering duration.**

::: info What if I don't have a GPU in my system?
Although it is called `GPU Mode`, it can be used on systems without a Graphics Card (GPU). Even with CPU integrated graphics, the performence boost is **huge**.
:::

Rsfrac automatically tries to initialize GPU Mode at startup, a log message will report the success of this operation. You can see this message at the bottom of the logs panel in this example:

![Preview](https://raw.githubusercontent.com/SkwalExe/rsfrac/main/assets/banner.png)

An indicator is also shown on the top left hand corner of the canvas (`GpuMode[on]`).

The `gpu` command must be used if you want to enable or disable GPU Mode for any reason.
It is always used without arguments.

## Tweaking GPU parameters ⚙

One GPU parameter can be changed with the `chunk_size (cs)` command.

This command changes the **maximum number of lines to render per GPU chunk.**
This is sometimes needed because the GPU can time out under a huge computational charge, thus returning **incorrect results.**
Lowering the maximum chunk size can help reduce render passes duration, consequently preventing timeouts and inconsistent results.

There are as of today no WGPU apis allowing to know if a GPU job finished or timed out. However, Rsfrac will try to detect GPU timeouts based on the job duration, and will reduce the chunk size automatically until the render succeeds.

## Arbitrary precision 🚫

> To lean more about arbitrary precision, you may read the [arbitraty precision guide](/arbitrary-precision).

The only disadvantage of GPU Mode is that is cannot be used simultaneously with **arbitrary precision arthmetics**. This prevents the usage of GPU Mode if you want to go deep in the rendered fractal.

GPU Mode can be used comfortably for scaling factors (zooms) up to `10^5`. If you want to go deeper you will have to disable GPU Mode with the `gpu` command. One symptom of low precision due to GPU Mode is the presence of blocky artefacts, as demonstrated on the screenshot below:

![example of low precision artefacts](/assets/low-precision.jpg)
