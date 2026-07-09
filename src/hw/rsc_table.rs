//! OpenAMP resource table for Linux `remoteproc` + virtio RPMsg.
//!
//! Layout follows Zephyr `nxp_resource_table.c` for imx8mp DDR firmware.

use core::mem::size_of;

const RSC_VENDOR_START: u32 = 128;
const RSC_VDEV: u32 = 3;
const VIRTIO_ID_RPMSG: u32 = 7;
const ADDR_ANY: u32 = 0xFFFF_FFFF;

const OFF_IMX: u32 = (size_of::<ResourceHdr>() + size_of::<[u32; 2]>()) as u32;
const OFF_VDEV: u32 = OFF_IMX + size_of::<FwRscImxVendor>() as u32;

#[repr(C, packed)]
struct ResourceHdr {
    ver: u32,
    num: u32,
    reserved: [u32; 2],
}

#[repr(C, packed)]
struct FwRscImxVendor {
    typ: u32,
    len: u32,
    magic_num: u32,
    version: u32,
    features: u32,
}

#[repr(C, packed)]
struct FwRscVdev {
    typ: u32,
    id: u32,
    notifyid: u32,
    dfeatures: u32,
    gfeatures: u32,
    config_len: u32,
    status: u8,
    num_of_vrings: u8,
    reserved: [u8; 2],
}

#[repr(C, packed)]
struct FwRscVring {
    da: u32,
    align: u32,
    num: u32,
    notifyid: u32,
    reserved: u32,
}

#[repr(C, packed)]
struct NxpResourceTable {
    hdr: ResourceHdr,
    offset: [u32; 2],
    imx_vs: FwRscImxVendor,
    vdev: FwRscVdev,
    vring0: FwRscVring,
    vring1: FwRscVring,
}

/// Linux parses this blob from the `.resource_table` ELF section.
#[unsafe(link_section = ".resource_table")]
#[used]
static RESOURCE_TABLE: NxpResourceTable = NxpResourceTable {
    hdr: ResourceHdr {
        ver: 1,
        num: 2,
        reserved: [0, 0],
    },
    offset: [OFF_IMX, OFF_VDEV],
    imx_vs: FwRscImxVendor {
        typ: RSC_VENDOR_START,
        len: size_of::<FwRscImxVendor>() as u32,
        magic_num: 0x6E78_7073, // 'nxps'
        version: 0,
        features: 0x1,
    },
    vdev: FwRscVdev {
        typ: RSC_VDEV,
        id: VIRTIO_ID_RPMSG,
        notifyid: 0,
        dfeatures: 1,
        gfeatures: 0,
        config_len: 0,
        status: 0,
        num_of_vrings: 2,
        reserved: [0, 0],
    },
    vring0: FwRscVring {
        da: ADDR_ANY,
        align: 16,
        num: 8,
        notifyid: 0,
        reserved: 0,
    },
    vring1: FwRscVring {
        da: ADDR_ANY,
        align: 16,
        num: 8,
        notifyid: 1,
        reserved: 0,
    },
};

/// Mutable pointer to the virtio status byte inside the resource table.
pub fn vdev_status_ptr() -> *mut u8 {
    unsafe {
        let base = &RESOURCE_TABLE as *const NxpResourceTable as *mut u8;
        base.add(OFF_VDEV as usize + 24)
    }
}

/// Host-patched TX vring device address (Linux fills `ADDR_ANY` at load).
pub fn vring0_da() -> u32 {
    vring_da(OFF_VDEV + size_of::<FwRscVdev>() as u32)
}

/// Host-patched RX vring device address.
pub fn vring1_da() -> u32 {
    vring_da(OFF_VDEV + size_of::<FwRscVdev>() as u32 + size_of::<FwRscVring>() as u32)
}

fn vring_da(off: u32) -> u32 {
    unsafe {
        let base = &RESOURCE_TABLE as *const NxpResourceTable as *const u8;
        core::ptr::read_unaligned(base.add(off as usize) as *const u32)
    }
}

/// VirtIO config status: driver OK.
pub const VIRTIO_CONFIG_STATUS_DRIVER_OK: u8 = 0x04;
