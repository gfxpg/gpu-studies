# Vulkan

## Prerequisites

Download VulkanSDK:
```bash
mkdir vulkan
cd vulkan
curl https://sdk.lunarg.com/sdk/download/1.1.97.0/linux/vulkansdk-linux-x86_64-1.1.97.0.tar.gz | tar xz
```

Set up environment variables in your shell profile:
```bash
VULKAN_SDK= ... # vulkan/1.1.97.0/x86_64
PATH=$PATH:$VULKAN_SDK/bin
LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$VULKAN_SDK/lib
VK_LAYER_PATH=$VULKAN_SDK/etc/explicit_layer.d
```
