# The bubas os

This name is a reference to the 🅱️-meme and my name.

## install disk_image from rustris
```bash
cargo install --git https://github.com/netthier/rustris disk_image
```

## Build and run in quemu
### find you OVMF
```bash
find / -name OVMF.fd
```

```
cargo kbuild --release && 
disk_image target/x86_64-unknown-uefi/release/bubas-os.efi && 
qemu-system-x86_64 -enable-kvm -bios [path from above]OVMF.fd -hda target/x86_64-unknown-uefi/release/bubas-os.img
```
