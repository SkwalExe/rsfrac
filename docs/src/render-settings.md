# Render Settings 💠 {#render-settings}

Rsfrac provides a multitude of render settings that can be changed to explore an **infinite** number of fractal variations.

> Use `help` followed by the name of any command to get a complete description of its usage.

### `frac`

The `frac` command allows you to select one of the 3 fractal algorithms.

Mandelbrot
![mandelbrot](/assets/mandelbrot.png)

Burning Ship
![burning ship](/assets/burning_ship.png)

Julia
![julia](/assets/julia.png)

### `color, smoothness (sm)`

The `color` and `smoothness (sm)` commands are used to customize the color palette used to display the fractal render.

> For a complete guide on how to use these commands, you may read the [Color Palettes section 🔗](/color-palettes)

### `max_iter (mi)`

> To learn more about technical concepts such as `divergent point`, you may read the [Fracal Logic section 🔗](/fractal-logic).

The `max_iter` command is used to change the maximum iteration count (MIC). The MIC is used to determine if a canvas point is convergent.
A convergent point will be marked as void and colored differently based on the settings.

The more you zoom in, the more divergence values will increase.
This may lead Rsfrac to mark a point as convergent, while it simply has a very high divergence point.
When this happens, the fractal will lose its details, which can look something like this:

![low max iter example](/assets/low_max_iter.png)

Increasing the MIC (for example with `mi + 40`) will increase the amount of details, which may look like this:

![good max iter example](/assets/good_max_iter.png)

### Void Fill

The void fill parameter doesn't have an associated command, but it can be changed by pressing `V` while the canvas is focused.
It will cycle through different void coloring methods.

![void fills](/assets/void_fills.gif)

### `click_mode (cm)`

The `click_mode (cm)` command allows you to assign specific actions to mouse buttons.
This enables you to adjust certain rendering parameters based on the location of your clicks on the canvas.
For example, clicking at the origin of the plane would set a parameter to `0+0j` or `0` depending on the type of the parameter, which can be Complex or Float.

The relevant click modes for this section are as follows:

::: info JuliaConstant

This click mode is used to set the Julia Constant (Complex).

In the Julia sequence: `Uₙ₊₁ = Uₙ² + C`, `C` is the Julia Constant.

> Use `frac info julia` to get more information about this constant.

:::

::: info MandelConstant

This click mode is used to set the Mandel Constant (Complex).

In the Mandelbrot sequence: `Uₙ₊₁ = Uₙ²+P`, `U₀` is the Mandelbrot Constant.

> Use `frac info mandel` to get more information about this constant.

:::

:::info Bailout

This click mode is used to set the Bailout Value (Float) for all fractals.

> To learn more about technical concepts such as `Bailout Value`, you may read the [Fracal Logic section 🔗](/fractal-logic).

:::
