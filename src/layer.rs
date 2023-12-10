use std::mem;

pub const PHY_LAYER_STATIC: u16 = 0x0100;
pub const PHY_LAYER_DYNAMIC: u16 = 0x0200;
pub const PHY_LAYER_BODY_PLAYER: u16 = 0x0300;
pub const PHY_LAYER_BODY_ALLY: u16 = 0x0400;
pub const PHT_LAYER_BODY_ENEMY: u16 = 0x0500;
pub const PHY_LAYER_SENSOR_STATIC: u16 = 0x0600;
pub const PHY_LAYER_SENSOR_DYNAMIC: u16 = 0x0700;

pub const PHY_SENSOR_PLAYER: u16 = 0x1;
pub const PHY_SENSOR_ALLY: u16 = 0x2;
pub const PHY_SENSOR_ENEMY: u16 = 0x4;
pub const PHY_SENSOR_ALL: u16 = 0x7;

pub const PHY_LAYER_TARGET: u16 = 0x0800;
pub const PHY_LAYER_HIT: u16 = 0x0900;

pub const PHY_TARGET_PLAYER: u16 = 0x1;
pub const PHY_TARGET_ALLY: u16 = 0x1;
pub const PHY_TARGET_ENEMY: u16 = 0x2;

pub const PHY_HIT_PLAYER: u16 = 0x1;
pub const PHY_HIT_ALLY: u16 = 0x1;
pub const PHY_HIT_ENEMY: u16 = 0x2;

const LAYER_STATIC_MASK: u32 = bit(PHY_LAYER_DYNAMIC) | bit(PHY_LAYER_BODY_PLAYER) | bit(PHY_LAYER_BODY_ALLY) | bit(PHT_LAYER_BODY_ENEMY);
const LAYER_DYNAMIC_MASK: u32 = bit(PHY_LAYER_STATIC) | bit(PHY_LAYER_BODY_PLAYER) | bit(PHY_LAYER_BODY_ALLY) | bit(PHT_LAYER_BODY_ENEMY);
const LAYER_BODY_PLAYER_MASK: u32 = bit(PHY_LAYER_STATIC) | bit(PHY_LAYER_DYNAMIC) | bit(PHT_LAYER_BODY_ENEMY);
const LAYER_BODY_FRIEND_MASK: u32 = bit(PHY_LAYER_STATIC) | bit(PHY_LAYER_DYNAMIC) | bit(PHT_LAYER_BODY_ENEMY);
const LAYER_BODY_ENEMY_MASK: u32 = bit(PHY_LAYER_STATIC) | bit(PHY_LAYER_DYNAMIC) | bit(PHY_LAYER_BODY_PLAYER) | bit(PHY_LAYER_BODY_ALLY);

const fn bit(layer: u16) -> u32 {
    return 1 << (layer >> 8);
}

fn rs_obj_obj_layer_filter(obj1: u16, obj2: u16) -> bool {
    let mut obj1 = obj1;
    let mut obj2 = obj2;
    if obj1 > obj2 {
        mem::swap(&mut obj1, &mut obj2);
    }
    let high1 = obj1 & 0xFF00;
    let low1 = obj1 & 0x00FF;
    let high2 = obj2 & 0xFF00;
    let low2 = obj2 & 0x00FF;
    return match high1 {
        PHY_LAYER_STATIC => LAYER_STATIC_MASK & bit(high2) != 0,
        PHY_LAYER_DYNAMIC => LAYER_DYNAMIC_MASK & bit(high2) != 0,
        PHY_LAYER_BODY_PLAYER => LAYER_BODY_PLAYER_MASK & bit(high2) != 0,
        PHY_LAYER_BODY_ALLY => LAYER_BODY_FRIEND_MASK & bit(high2) != 0,
        PHT_LAYER_BODY_ENEMY => LAYER_BODY_ENEMY_MASK & bit(high2) != 0,
        PHY_LAYER_SENSOR_STATIC | PHY_LAYER_SENSOR_DYNAMIC => {
            true
            // if high2 == PHY_LAYER_BODY_PLAYER {
            //     low1 & PHY_SENSOR_PLAYER != 0
            // } else if high2 == PHY_LAYER_BODY_ALLY {
            //     low1 & PHY_SENSOR_ALLY != 0
            // } else if high2 == PHT_LAYER_BODY_ENEMY {
            //     low1 & PHY_SENSOR_ENEMY != 0
            // } else {
            //     false
            // }
        }
        PHY_LAYER_TARGET => (high2 == PHY_LAYER_HIT) & (low1 & low2 != 0),
        PHY_LAYER_HIT => (high2 == PHY_LAYER_TARGET) & (low1 & low2 != 0),
        _ => false,
    };
}

const PHY_BP_LAYER_STATIC: u8 = 0x00;
const PHY_BP_LAYER_MOVE: u8 = 0x01;
const PHY_BP_LAYER_HIT: u8 = 0x02;

const BP_LAYER_STATIC_MASK: u32 =
    bit(PHY_LAYER_DYNAMIC) | bit(PHY_LAYER_BODY_PLAYER) | bit(PHY_LAYER_BODY_ALLY) | bit(PHT_LAYER_BODY_ENEMY);
const BP_LAYER_MOVE_MASK: u32 = bit(PHY_LAYER_STATIC)
    | bit(PHY_LAYER_DYNAMIC)
    | bit(PHY_LAYER_BODY_PLAYER)
    | bit(PHY_LAYER_BODY_ALLY)
    | bit(PHT_LAYER_BODY_ENEMY)
    | bit(PHY_LAYER_SENSOR_STATIC)
    | bit(PHY_LAYER_SENSOR_DYNAMIC);
const BP_LAYER_HIT_MASK: u32 = bit(PHY_LAYER_TARGET) | bit(PHY_LAYER_HIT);

fn rs_obj_to_bp_layer(obj: u16) -> u8 {
    match obj & 0xFF00 {
        PHY_LAYER_STATIC => PHY_BP_LAYER_STATIC,
        PHY_LAYER_DYNAMIC => PHY_BP_LAYER_MOVE,
        PHY_LAYER_BODY_PLAYER => PHY_BP_LAYER_MOVE,
        PHY_LAYER_BODY_ALLY => PHY_BP_LAYER_MOVE,
        PHT_LAYER_BODY_ENEMY => PHY_BP_LAYER_MOVE,
        PHY_LAYER_SENSOR_STATIC => PHY_BP_LAYER_STATIC,
        PHY_LAYER_SENSOR_DYNAMIC => PHY_BP_LAYER_MOVE,
        PHY_LAYER_TARGET => PHY_BP_LAYER_HIT,
        PHY_LAYER_HIT => PHY_BP_LAYER_HIT,
        _ => 0,
    }
}

fn rs_obj_bp_layer_filter(obj: u16, bp: u8) -> bool {
    match bp {
        PHY_BP_LAYER_STATIC => BP_LAYER_MOVE_MASK & bit(obj) != 0,
        PHY_BP_LAYER_MOVE => BP_LAYER_MOVE_MASK & bit(obj) != 0,
        PHY_BP_LAYER_HIT => BP_LAYER_HIT_MASK & bit(obj) != 0,
        _ => false,
    }
}

fn rs_bp_layer_name(bp: u8) -> &'static str {
    match bp {
        PHY_BP_LAYER_STATIC => "Static",
        PHY_BP_LAYER_MOVE => "Move",
        PHY_BP_LAYER_HIT => "Hit",
        _ => "Unknown",
    }
}

#[cxx::bridge()]
mod ffi {
    extern "Rust" {
        #[cxx_name = "RsObjObjLayerFilter"]
        fn rs_obj_obj_layer_filter(a: u16, b: u16) -> bool;
        #[cxx_name = "RsObjToBpLayer"]
        fn rs_obj_to_bp_layer(layer: u16) -> u8;
        #[cxx_name = "RsObjBpLayerFilter"]
        fn rs_obj_bp_layer_filter(l: u16, bpl: u8) -> bool;
        #[cxx_name = "RsBpLayerName"]
        fn rs_bp_layer_name(bp: u8) -> &'static str;
    }
}
