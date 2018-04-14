# Matrix multiplication

## Gotchas

On Intel's iGPUs, the kernel may not be executed over the full global work range,
which would result in validation errors (some of the values ending up being 0).
An execution counter atomically incremented in the kernel would show that the
number of times the kernel is ran is indeed less than the global work size.

In my case, this was related to GPU hang checks executed by the driver. Run
`dmesg | grep 'timed out'`, and if you see messages like 
`asynchronous wait on fence i915:[...] timed out`, then that's most likely the problem.

The solution is disabling the checks by running
`echo -n 0 > /sys/module/i915/parameters/enable_hangcheck`. Note that with the check
disabled, if the kernel really hangs you won't be able to do anything but reboot.
