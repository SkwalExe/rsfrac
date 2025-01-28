# Arbitrary Precision 🔢 {#arbitrary-precision}

In computer science, numbers of generally stored in a memory segment scaling up to 16 bytes. This is sufficient most of the time, however to explore fractals at very high depths, more precision is needed.

Without Arbitrary Precision, you can comfortably explore fractals at scaling factors (zooms) up to `10^5`. One symptom of insufficient precision is the presence of blocky artefacts, as demonstrated on the screenshot below:

![example of low precision artefacts](/assets/low-precision.jpg)

---

Arbitrary Precision allows you to explore fractals at very high deepths by storing numbers on custom-length memory segments scaling up to **8192 bytes**.

You can adjust the precision used during renders using the `prec` command, which can be used with:

- `no arguments`: it will display the current precision is **BITS**
- `[precision]`: will set the precision to the specified bit count
- `+/- [increment]`: will in/decrement the precision by the specified bit count.

For example, `prec + 10` will increment the precision by 10 bits, and `prec 512` will set the precision to 512 bits (64 bytes).

## GPU Mode 🚫

> For more information about GPU Mode, you may read the [GPU Mode guide](/gpu-mode)

Arbitrary Precision is always enabled, but is ignored when GPU Mode is active. To benefit from it, you must disable GPU Mode with the `gpu` command. **Note that disabling GPU Mode will make render much slower.**
