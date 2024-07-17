use crate::cube::*;
use bevy::prelude::*; //导入 Bevy 引擎的核心和拾取模块。
use bevy_mod_picking::prelude::*;
use std::collections::VecDeque; //导入双端队列
use std::f32::consts::FRAC_PI_2; //导入常量 PI、TAU 和 FRAC_PI_2（π/2）
use std::f32::consts::PI;
use std::f32::consts::TAU;

#[derive(Debug, Clone, Copy)]
pub enum SideRotation {
    //定义了 90 度和 180 度的顺时针和逆时针旋转。
    Clockwise90,
    Clockwise180,
    Counterclockwise90,
}

#[derive(Debug, Clone, Copy)]
pub enum Axis {
    //定义了 X、Y、Z 三个轴。
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy, Event)]
pub struct SideMoveEvent {
    //定义了魔方的侧面旋转事件，包含旋转的面和旋转方向。
    // 旋转的面，对应固定的x/y/z坐标值
    pub side: (Axis, f32),
    // 旋转
    pub rotate: SideRotation,
}

#[derive(Debug, Resource)]
pub struct SideMoveQueue(pub VecDeque<SideMoveEvent>); //包含一个 VecDeque 用于存储 SideMoveEvent

#[derive(Debug, Resource)]
pub struct MouseDraggingRecorder {
    //记录鼠标拖动开始的位置和选中的魔方块
    pub start_pos: Option<Vec3>,
    pub piece: Option<Entity>,
}

impl MouseDraggingRecorder {
    //实现了 clear 方法，用于重置记录。
    pub fn clear(&mut self) {
        self.start_pos = None;
        self.piece = None;
    }
}
