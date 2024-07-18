//一个用于 3D 魔方的相机系统，利用 Bevy 引擎处理相机的设置、缩放和移动。
use std::f32::consts::TAU; //导入常量 TAU（等于 2π）

use crate::cube::*; //导入项目中的 cube 和 moving 模块
use crate::moving::*;
use bevy::input::mouse::MouseMotion; //导入 Bevy 引擎的鼠标输入和核心预定义模块。
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy_mod_picking::backends::raycast::RaycastPickable; //导入 Bevy 的选取模块，用于处理鼠标选取。

pub fn setup_camera(mut commands: commands){
    commands.spawn(Camera3dBundle{//将相机放置在 (5.0, 5.0, 5.0) 的位置。看向场景的原点 (0, 0, 0),将 Vec3::Y 设为向上的方向。
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
    .insert(RaycastPickable::default());
}
// TODO 平滑放大缩小
pub fn zoom_camera(
    //处理鼠标滚轮事件，实现相机的放大和缩小。根据滚轮事件的单位（行或像素），调整相机的位置，以实现缩放效果。
    mut scroll_evr: EventReader<MouseWheel>,//读取 MouseWheel 事件的 EventReader
    mut q_camera: Query<&mut Transform, With<Camera>>,//查询具有 Camera 组件的实体的 Query
    cube_settings: Res<CubeSettings>,
) {
    for ev in scroll_evr.read(){
        let mut transform = q_camera.single_mut();
        match  ev.unit {
            MouseScrollUnit::Line =>{//传统的滚轮鼠标
                if  ev.x+ev.y >0{
                    transform.translation.x = cube_settings.camera_zoom_speed * transform.translation.x;
                    transform.translation.y = cube_settings.camera_zoom_speed * transform.translation.y;
                    transform.translation.z = cube_settings.camera_zoom_speed * transform.translation.z;
                }else{
                    transform.translation.x = transform.translation.x / cube_settings.camera_zoom_speed;
                    transform.translation.y = transform.translation.y / cube_settings.camera_zoom_speed;
                    transform.translation.z = transform.translation.z / cube_settings.camera_zoom_speed;
                }
            } 
            MouseScrollUnit::Pixel=>{//适用于需要精细滚动的应用程序或界面
                if  ev.x+ev.y >0{
                    transform.translation.x = cube_settings.camera_zoom_speed * transform.translation.x;
                    transform.translation.y = cube_settings.camera_zoom_speed * transform.translation.y;
                    transform.translation.z = cube_settings.camera_zoom_speed * transform.translation.z;
                }else{
                    transform.translation.x = transform.translation.x / cube_settings.camera_zoom_speed;
                    transform.translation.y = transform.translation.y / cube_settings.camera_zoom_speed;
                    transform.translation.z = transform.translation.z / cube_settings.camera_zoom_speed;
                }
            } 
        }
    }
}

pub fn move_camera(//函数处理鼠标移动事件，实现相机的平移和旋转。当鼠标左键按下时，根据鼠标移动的方向和距离，计算相应的旋转角度，使相机围绕原点旋转
    mut q_camera: Query<&mut Transform, With<Camera>>,
    mut motion_evr: EventReader<MouseMotion>,
    buttons: Res<ButtonInput<MouseButton>>,
    recorder: Res<MouseDraggingRecorder>,
) {
    if buttons.pressed(input: MouseButton::Left){
        if recorder.piece.is_none() || recorder.start_pos.is_none(){
            for motion in motion_evr.read(){
                // motion.delta.x 鼠标左滑为负、右滑为正
                // motion.delta.y 鼠标上滑为负、下滑为正
                for mut transform in &mut q_camera{
                    if motion.delta.x.abs()>0.001{
                        //水平转动y轴
                        let max = transform.translation.x.abs().max(transform.translation.y.abs()).max(transform.translation.z.abs());
                        let quat = Quat::from_euler(
                            euler: EulerRot::XYZ,
                            a: 0,
                            b: 0.0002 * -motion.delta.x * max * TAU,//与上下滑动保持同步
                            c:0,
                        );
                        transform.rotate_around(Vec3::ZERO, quat);
                    }
                    if motion.delta.y.abs() > 0.001{
                        //垂直转动 xz轴
                        let quat = Quat::from_euler(
                            euler: EulerRot::XYZ,
                            a: 0.002* -motion.delta.y * transform.translation.z * TAU,
                            b: 0,
                            c: 0.002* motion.delta.y * transform.translation.x * TAU,
                        );
                        transform.rotate_around(Vec3::ZERO, quat);

                    }
                }
            }
        }
 
    }
    montion_evr.clear();
}
