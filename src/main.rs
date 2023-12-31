use bevy::{prelude::*, sprite::MaterialMesh2dBundle, render::{camera::{ScalingMode, RenderTarget}, render_resource::{TextureDescriptor, Extent3d, TextureDimension, TextureFormat, TextureUsages}, view::RenderLayers}, window::{WindowMode}};

const RESOLUTION: (f32, f32) = (200.0, 200.0);
const SPEED: f32 = 1.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Window".into(),

                    #[cfg(not(target_arch = "wasm32"))]
                    mode: WindowMode::Fullscreen,

                    resolution: RESOLUTION.into(),
                    canvas: Some("#game".into()),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest()))
        .add_startup_system(setup)
        .add_system(movement)
        .add_system(movement_camera)
        .run();
}

#[derive(Component)]
struct MovementEnabled;

#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>
    ) {
    
    let gap = 150.0;

    commands.insert_resource(ClearColor(Color::hex("#313131").unwrap()));

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::splat(20.0)),
            ..default()
        },
        transform: Transform::from_xyz(-gap / 2.0, 0.0, 0.0),
        ..default()
    });

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::GREEN,
            custom_size: Some(Vec2::splat(20.0)),
            ..default()
        },
        transform: Transform::from_xyz(gap / 2.0, 0.0, 0.0),
        ..default()
    });

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(15.0).into()).into(),
        material: materials.add(ColorMaterial::from(Color::BLUE)),
        transform: Transform::from_translation(Vec3::new(0.0, 50.0, 1.0)),
        ..default()
    }).insert(MovementEnabled);


    let size = Extent3d {
        width: RESOLUTION.0 as u32 ,
        height: RESOLUTION.1 as u32,
        ..Default::default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            view_formats: &[TextureFormat::Rgba8Unorm],
            usage: TextureUsages::TEXTURE_BINDING 
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT
        },
        ..Default::default()
    };

    image.resize(size);

    let image_handle = images.add(image);

    commands.spawn(Camera2dBundle {
        camera: Camera {
            target: RenderTarget::Image(image_handle.clone()),
            ..Default::default()
        },
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::Fixed { width: RESOLUTION.0, height: RESOLUTION.1 },
            ..Default::default()
        },
        ..Default::default()
    }).insert(MainCamera);

    commands.spawn(Camera2dBundle::default()).insert(RenderLayers::none());

    commands.spawn(ImageBundle {
        style: Style {
            size: Size::new(Val::Auto, Val::Auto),
            margin: UiRect::all(Val::Auto),
            ..default()
        },
        image: image_handle.into(),
        ..default()
    });
}

fn movement(
    mut object: Query<&mut Transform, With<MovementEnabled>>,
    keys: Res<Input<KeyCode>>
    ) {
    let mut transform = object.single_mut();
    
    let mut translation = Vec3::ZERO;

    if keys.pressed(KeyCode::Left) {
        translation.x -= 1.0;
    }

    if keys.pressed(KeyCode::Right) {
        translation.x += 1.0;
    }

    if keys.pressed(KeyCode::Down) {
        translation.y -= 1.0;
    }

    if keys.pressed(KeyCode::Up) {
        translation.y += 1.0;
    }

    translation = translation.normalize_or_zero() * SPEED;
    transform.translation += translation;
}


fn movement_camera(
    mut object: Query<&mut Transform, With<MainCamera>>,
    keys: Res<Input<KeyCode>>
    ) {
    let mut transform = object.single_mut();
    
    let mut translation = Vec3::ZERO;

    if keys.pressed(KeyCode::A) {
        translation.x -= 1.0;
    }

    if keys.pressed(KeyCode::D) {
        translation.x += 1.0;
    }

    if keys.pressed(KeyCode::S) {
        translation.y -= 1.0;
    }

    if keys.pressed(KeyCode::W) {
        translation.y += 1.0;
    }

    translation = translation.normalize_or_zero() * SPEED;
    transform.translation += translation;

}
