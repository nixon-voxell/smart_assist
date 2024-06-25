use bevy::{math::DVec2, prelude::*, ui::FocusPolicy, winit::WinitSettings};
use bevy_vello_graphics::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(VelloGraphicsPlugin)
        .insert_resource(WinitSettings::desktop_app()) // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .add_systems(Startup, setup)
        // .add_systems(Update, switch_logic)
        .run()
}

const OFF_BUTTON_COLOR: Color = Color::RED;
const ON_BUTTON_COLOR: Color = Color::GREEN;

#[derive(Component)]
struct ButtonState {
    is_on: bool,
    original_color: Color,
}

#[derive(Bundle, Clone, Debug)]
pub struct VelloButtonBundle {
    /// Describes the logical size of the node
    pub node: Node,
    /// Marker component that signals this node is a button
    pub button: Button,
    /// Styles which control the layout (size and position) of the node and it's children
    /// In some cases these styles also affect how the node drawn/painted.
    pub style: Style,
    pub vello_rect: VelloRect,
    /// Describes whether and how the button has been interacted with by the input
    pub interaction: Interaction,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The background color, which serves as a "fill" for this node
    ///
    /// When combined with `UiImage`, tints the provided image.
    pub background_color: BackgroundColor,
    /// The color of the Node's border
    pub border_color: BorderColor,
    /// The image of the node
    pub image: UiImage,
    /// The transform of the node
    ///
    /// This component is automatically managed by the UI layout system.
    /// To alter the position of the `ButtonBundle`, use the properties of the [`Style`] component.
    pub transform: Transform,
    /// The global transform of the node
    ///
    /// This component is automatically updated by the [`TransformPropagate`](`bevy_transform::TransformSystem::TransformPropagate`) systems.
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
    /// Indicates the depth at which the node should appear in the UI
    pub z_index: ZIndex,
}

impl Default for VelloButtonBundle {
    fn default() -> Self {
        Self {
            focus_policy: FocusPolicy::Block,
            node: Default::default(),
            button: Default::default(),
            style: Default::default(),
            vello_rect: Default::default(),
            border_color: BorderColor(Color::NONE),
            interaction: Default::default(),
            background_color: Default::default(),
            image: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
            z_index: Default::default(),
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                VelloSceneBundle::default(),
            ));
            parent
                .spawn(VelloButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        // border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    vello_rect: VelloRect { // not showing rect
                        size: DVec2::new(100.0, 200.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(ButtonState {
                    is_on: false,
                    original_color: OFF_BUTTON_COLOR,
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Off",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

fn darken_color(color: Color, factor: f32) -> Color {
    let [r, g, b, a] = color.as_rgba_f32();
    Color::rgba(
        (r - factor).max(0.0),
        (g - factor).max(0.0),
        (b - factor).max(0.0),
        a,
    )
}

fn switch_logic(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &mut ButtonState,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children, mut button_state) in
        &mut interaction_query
    {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                button_state.is_on = !button_state.is_on;

                if button_state.is_on {
                    text.sections[0].value = "On".to_string();
                    *color = ON_BUTTON_COLOR.into();
                    border_color.0 = ON_BUTTON_COLOR;
                    button_state.original_color = ON_BUTTON_COLOR;
                } else {
                    text.sections[0].value = "Off".to_string();
                    *color = OFF_BUTTON_COLOR.into();
                    border_color.0 = OFF_BUTTON_COLOR;
                    button_state.original_color = OFF_BUTTON_COLOR;
                }

                // trigger to controller/middleware here
            }
            Interaction::Hovered => {
                *color = darken_color(button_state.original_color, 0.25).into();
                border_color.0 = darken_color(button_state.original_color, 0.25);
            }
            Interaction::None => {
                *color = button_state.original_color.into();
                border_color.0 = button_state.original_color;
            }
        }
    }
}
