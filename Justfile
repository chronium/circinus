target := "x86_64"
mem := "512M"

img := "os.img"
drive := "if=none,id=dsk0,format=raw,file=build/" + img
# drive := "id=dsk0,format=raw,file=build/" + img

virtio-net := "virtio-net,netdev=net0,disable-legacy=on,disable-modern=off"
dev := "user,id=net0"
pcap := "filter-dump,id=filter0,netdev=net0,file=virtio-net.pcap"

virtio-blk := "virtio-blk-pci,drive=dsk0,disable-legacy=on,disable-modern=off"

# + " -device " + virtio-net

# qemu-args := " -serial stdio" + " -m " + mem + " -drive " + drive + " -no-reboot -d cpu_reset -s" + " -netdev " + dev + " -object " + pcap + " -device " + virtio-blk
#  
qemu-args := " -serial stdio" + " -m " + mem + " -drive " + drive + " -no-reboot -d cpu_reset -s" + " -device " + virtio-net + " -netdev " + dev + " -object " + pcap + " -device " + virtio-blk

limine := "extern/limine/build/bin"

clean:
    cargo clean
    rm -f build/*
    sudo umount -R isotmp/ | cat
    sudo rm -rf isotmp/
    rm -f loopback_dev
    echo "Clean is complete"

build release:
    #!/usr/bin/env bash
    set -e
    if { [ release != "debug" ] && [ release != "release" ] ;} then \
        echo Unknown build mode \"{{release}}\";\
        exit 1; \
    fi;

    cargo build {{ if release == "debug" { "" } else { "--release" } }} --target kernel/arch/x86_64/x86_64.json
    cp target/{{target}}/{{release}}/kernel build/kernel.elf

    nm build/kernel.elf | rustfilt | awk '{ $2=""; print $0 }' > build/kernel.sym
    python3 ./embed-symbol-table.py build/kernel.sym build/kernel.elf

image name:
    #!/usr/bin/env sh
    set -e
    path="build/{{name}}"

    dd if=/dev/zero of=$path bs=1M count=64
    parted -s $path mklabel gpt
    parted -s $path mkpart primary 2048s 100%
    sudo losetup -Pf --show $path >loopback_dev
    mkdir -p isotmp
    sudo partprobe $(cat loopback_dev)
    sudo mkfs.ext2 $(cat loopback_dev)p1
    sudo mount $(cat loopback_dev)p1 isotmp/
    sudo mkdir -p isotmp/boot
    sudo cp -Rf build/kernel.elf isotmp/boot
    sudo cp -Rf build/kernel.sym isotmp/boot
    sudo cp -Rf {{limine}}/limine.sys isotmp/boot
    sudo cp -Rf root/* isotmp/
    sync
    sudo umount isotmp/
    sudo losetup -d $(cat loopback_dev)
    ./{{limine}}/limine-install $path
    echo "HDD is complete"

dumpext2 name=img:
    #!/usr/bin/env sh
    set -e
    path="build/{{name}}"

    sudo losetup -Pf --show $path >loopback_dev
    sudo dumpe2fs $(cat loopback_dev)p1
    sudo losetup -d $(cat loopback_dev)

debugfs name=img:
    #!/usr/bin/env sh
    set -e
    path="build/{{name}}"
    
    sudo losetup -Pf --show $path >loopback_dev
    sudo debugfs $(cat loopback_dev)p1
    sudo losetup -d $(cat loopback_dev)

limine:
    #!/usr/bin/env bash
    pushd ./extern/limine
    make
    popd

run release="debug": (build release) (image img)
    qemu-system-{{target}} -cpu Haswell {{qemu-args}}

kvm release="debug": (build release) (image img)
    sudo qemu-system-{{target}} -enable-kvm -cpu host {{qemu-args}}