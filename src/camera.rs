//一个用于 3D 魔方的相机系统，利用 Bevy 引擎处理相机的设置、缩放和移动。
use std::f32::consts::TAU; //导入常量 TAU（等于 2π）

use crate::cube::*; //导入项目中的 cube 和 moving 模块
use crate::moving::*;
use bevy::input::mouse::MouseMotion; //导入 Bevy 引擎的鼠标输入和核心预定义模块。
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy_mod_picking::backends::raycast::RaycastPickable; //导入 Bevy 的选取模块，用于处理鼠标选取。

// TODO 平滑放大缩小 参考 https://github.com/cart/card_combinator/blob/main/src/game/camera.rs
pub fn zoom_camera(
    //处理鼠标滚轮事件，实现相机的放大和缩小。根据滚轮事件的单位（行或像素），调整相机的位置，以实现缩放效果。
    mut scroll_evr: EventReader<MouseWheel>,
    mut q_camera: Query<&mut Transform, With<Camera>>,
    cube_settings: Res<CubeSettings>,
) {
}
