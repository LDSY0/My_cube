use crate::moving::{self, *};
use bevy::prelude::*; //导入 Bevy 引擎的所有预定义模块。
use bevy_mod_picking::backends::raycast::RaycastPickable; //导入 Bevy 的选取和拖拽模块，用于处理用户的鼠标交互。
use bevy_mod_picking::events::Move;
use bevy_mod_picking::prelude::DragEnd;
use bevy_mod_picking::prelude::DragStart;
use bevy_mod_picking::prelude::On;
use bevy_mod_picking::prelude::Pointer;
use bevy_mod_picking::PickableBundle;
use rand::seq::SliceRandom; //导入随机数生成库。
use rand::Rng;
use std::f32::consts::FRAC_PI_2; //导入浮点数常量。
use std::f32::consts::PI;

#[derive(Debug, Component)] //标记该结构体为一个 Bevy 组件，使其可以附加到实体上
pub struct MovablePiece {
    pub axis: moving::Axis,   //指示魔方块将围绕哪个轴旋转
    pub rotate: SideRotation, //旋转的方向（顺时针90度、顺时针180度、逆时针90度）
    pub left_angle: f32, //魔方块剩余的旋转角度,用于跟踪旋转动画的进度。当 left_angle 为0时，表示旋转完成
}

#[derive(Debug, Component, Reflect, Default, Clone, Copy)]
#[reflect(Component)]
pub struct Piece {
    pub init_pos: Vec3, //包含初始位置和大小的结构体，用于表示魔方块。
    pub size: f32,
}

impl Piece {
    //提供方法来判断魔方块的各个面是否存在。
    pub fn has_up_face(&self) -> bool {
        self.init_pos.y == 1.0
    }
    pub fn has_down_face(&self) -> bool {
        self.init_pos.y == -1.0
    }
    pub fn has_left_face(&self) -> bool {
        self.init_pos.x == -1.0
    }
    pub fn has_right_face(&self) -> bool {
        self.init_pos.x == 1.0
    }
    pub fn has_front_face(&self) -> bool {
        self.init_pos.z == 1.0
    }
    pub fn has_back_face(&self) -> bool {
        self.init_pos.z == -1.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlayMode {
    // 练习模式
    Practice,
    // 计时模式
    Timekeeping,
}

/// 魔方设置
#[derive(Debug, Resource)]
pub struct CubeSettings {
    //包含魔方的设置，如阶数、块大小、旋转速度、各面颜色、游戏模式和相机缩放速度。
    // 几阶魔方
    pub cube_order: u8,
    //魔方块大小
    pub piece_size: f32,
    //滑动速度
    pub rotate_speed: f32,
    //颜色
    pub front_color: Color,
    pub back_color: Color,
    pub up_color: Color,
    pub down_color: Color,
    pub left_color: Color,
    pub right_color: Color,
    //游玩模式
    pub play_mode: PlayMode,
    //相机缩放速度
    pub camera_zoom_speed: f32,
}

impl Default for CubeSettings {
    fn default() -> Self {
        Self {
            cube_order: 3,
            piece_size: 1.0,
            rotate_speed: 1.0,
            front_color: Color::GREEN,
            back_color: Color::RED,
            up_color: Color::WHITE,
            down_color: Color::ORANGE,
            left_color: Color::BLUE,
            right_color: Color::YELLOW,
            play_mode: PlayMode::Practice,
            camera_zoom_speed: 1.01,
        }
    }
}

// 重置魔方
#[derive(Debug, Default, Event)] //定义事件
pub struct ResetEvent;

// 打乱魔方
#[derive(Debug, Default, Event)]
pub struct ScrambleEvent;

pub fn setup_cube(
    mut commands: Commands,                          //创建一个新的魔方实体
    mut meshes: ResMut<Assets<Mesh>>,                //添加一个新的魔方网格
    mut materials: ResMut<Assets<StandardMaterial>>, //添加一个新的魔方材质
    cube_settings: Res<CubeSettings>,                //获取魔方的大小和颜色设置
) {
    create_cube(&mut commands, &mut meshes, &mut materials, &cube_settings);
}

pub fn create_cube(
    commands: &mut Commands,                          //创建一个新的魔方实体
    meshes: &mut ResMut<Assets<Mesh>>,                //添加一个新的魔方网格
    materials: &mut ResMut<Assets<StandardMaterial>>, //添加一个新的魔方材质
    cube_settings: &Res<CubeSettings>,                //获取魔方的大小和颜色设置
) {
    for x in [-1.0, 0.0, 1.0] {
        for y in [-1.0, 0.0, 1.0] {
            for z in [-1.0, 0.0, 1.0] {
                let piece = Piece {
                    init_pos: Vec3::new(x, y, z),
                    size: cube_settings.piece_size,
                };
                commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0).mesh()), //创建一个尺寸为 1.0 x 1.0 x 1.0 的立方体对象,将生成的网格数据添加到 meshes 资源管理器中
                        material: materials.add(Color::BLACK), //添加一个黑色材质到 materials 资源管理器，并将其分配给实体的材质组件。
                        transform: Transform::from_translation(Vec3::new(x, y, z)), //设置实体的初始位置为 (x, y, z)。
                        ..default()
                    })
                    .insert(piece) //为实体插入 Piece 组件，该组件包含初始位置和大小的信息
                    .insert(PickableBundle::default()) //为实体插入 PickableBundle 组件，使其在使用 Bevy 的选择插件（如 bevy_mod_picking）时可以被选中
                    .insert(RaycastPickable::default()) //为实体插入 RaycastPickable 组件，使其在使用光线投射插件时可以被拾取
                    .insert(On::<Pointer<DragStart>>::run(handle_drag_start)) //为实体插入一个事件监听器，当光标拖动开始事件 (Pointer<DragStart>) 触发时，运行 handle_drag_start 函数。
                    .insert(On::<Pointer<Move>>::run(handle_move)) //为实体插入一个事件监听器，当光标移动事件 (Pointer<Move>) 触发时，运行 handle_move 函数。
                    .insert(On::<Pointer<DragEnd>>::run(handle_drag_end)) //为实体插入一个事件监听器，当光标拖动结束事件 (Pointer<DragEnd>) 触发时，运行 handle_drag_end 函数。
                    .with_children(|parent: &mut ChildBuilder| {
                        //贴纸
                        spawn_stickers(parent, piece, meshes, materials, cube_settings);
                    });
            }
        }
    }
}

pub fn spawn_stickers(
    //函数为每个魔方块生成相应的贴纸，根据块的位置确定贴纸的颜色和位置。
    parent: &mut ChildBuilder,
    piece: Piece,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    cube_settings: &CubeSettings,
) {
    let sticker_size = 0.9 * cube_settings.piece_size;

    if piece.has_up_face() {
        let mut transform =
            Transform::from_translation(Vec3::new(0.0, 0.5 * cube_settings.piece_size + 0.01, 0.0));
        parent.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(sticker_size, 0.01, sticker_size)),
            material: materials.add(StandardMaterial {
                base_color: cube_settings.up_color,
                unlit: true,
                ..default()
            }),
            transform,
            ..Default::default()
        });
    }

    if piece.has_down_face() {
        let mut transform = Transform::from_translation(Vec3::new(
            0.0,
            -0.5 * cube_settings.piece_size - 0.01,
            0.0,
        ));
        parent.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(sticker_size, 0.01, sticker_size)),
            material: materials.add(StandardMaterial {
                base_color: cube_settings.down_color,
                unlit: true,
                ..default()
            }),
            transform,
            ..Default::default()
        });
    }

    if piece.has_front_face() {
        let mut transform =
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.5 * cube_settings.piece_size + 0.01));
        transform.rotate_x(FRAC_PI_2); //绕x顺时针90度
        parent.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(sticker_size, 0.01, sticker_size)),
            material: materials.add(StandardMaterial {
                base_color: cube_settings.front_color,
                unlit: true,
                ..default()
            }),
            transform,
            ..Default::default()
        });
    }

    if piece.has_back_face() {
        let mut transform = Transform::from_translation(Vec3::new(
            0.0,
            0.0,
            -0.5 * cube_settings.piece_size - 0.01,
        ));
        transform.rotate_x(-FRAC_PI_2); //绕x逆时针90度
        parent.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(sticker_size, 0.01, sticker_size)),
            material: materials.add(StandardMaterial {
                base_color: cube_settings.back_color,
                unlit: true,
                ..default()
            }),
            transform,
            ..Default::default()
        });
    }

    if piece.has_left_face() {
        let mut transform = Transform::from_translation(Vec3::new(
            -0.5 * cube_settings.piece_size - 0.01,
            0.0,
            0.0,
        ));
        transform.rotate_z(FRAC_PI_2);
        parent.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(sticker_size, 0.01, sticker_size)),
            material: materials.add(StandardMaterial {
                base_color: cube_settings.left_color,
                unlit: true,
                ..default()
            }),
            transform,
            ..Default::default()
        });
    }

    if piece.has_right_face() {
        let mut transform =
            Transform::from_translation(Vec3::new(0.5 * cube_settings.piece_size + 0.01, 0.0, 0.0));
        transform.rotate_z(-FRAC_PI_2);
        parent.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(sticker_size, 0.01, sticker_size)),
            material: materials.add(StandardMaterial {
                base_color: cube_settings.right_color,
                unlit: true,
                ..default()
            }),
            transform,
            ..Default::default()
        });
    }
}

pub fn reset_cube(
    //响应 ResetEvent，重新生成魔方。
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cube_settings: Res<CubeSettings>,
    mut events: EventReader<ResetEvent>,
    q_pieces: Query<Entity, With<Piece>>,
) {
    for _ in events.read() {
        // 移除原有魔方
        for piece in &q_pieces {
            commands.entity(piece).despawn_recursive();
        }
        // 重建魔方
        create_cube(&mut commands, &mut meshes, &mut materials, &cube_settings);
    }
}

pub fn scramble_cube(
    //响应 ScrambleEvent，随机生成旋转操作，将它们加入旋转队列中以实现魔方打乱。
    mut events: EventReader<ScrambleEvent>,
    mut side_move_queue: ResMut<SideMoveQueue>,
) {
    for _ in events.read() {
        for _ in 0..5 {
            let axis_value = vec![-1.0f32, 0.0, 1.0]
                .choose(&mut rand::thread_rng())
                .unwrap()
                .clone();
            let axis = match rand::thread_rng().gen_range(0..3) {
                0 => moving::Axis::X,
                1 => moving::Axis::Y,
                2 => moving::Axis::Z,
                _ => moving::Axis::X,
            };
            // let axis = moving::Axis::Y;
            let rotate = match rand::thread_rng().gen_range(0..3) {
                0 => SideRotation::Clockwise90,
                1 => SideRotation::Clockwise180,
                2 => SideRotation::Counterclockwise90,
                _ => SideRotation::Clockwise90,
            };
            side_move_queue.0.push_back(SideMoveEvent {
                side: (axis, axis_value),
                rotate,
            })
        }
    }
}
