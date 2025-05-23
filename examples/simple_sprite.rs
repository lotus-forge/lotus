//! This example is a simple show off about rendering sprites.
//! The sprite is rendered inside the 'setup' function.
//! And is mutated inside the 'update' function, having its 'y' position updated at each frame.
//! This example serves as an example of the basic use of the velocity component when mutating transformation matrices.

use lotus_engine::*;

your_game!(
    WindowConfiguration::default(),
    setup,
    update
);

fn setup(context: &mut Context) {
    let sprite: Sprite = Sprite::new("textures/lotus_pink_256x256.png".to_string());

    context.commands.spawn(
        vec![
            Box::new(sprite),
            Box::new(Transform::new_simple(Position::new(Vector2::new(-0.50, -0.50), Strategy::Normalized))),
            Box::new(Velocity::new(Vector2::new(0.50, 0.50)))
        ]
    );
}

fn update(context: &mut Context) {
    let mut query: Query = Query::new(&context.world).with::<Sprite>();
    let results: Vec<Entity> = query.entities_with_components().unwrap();
    let entity: &Entity = results.first().unwrap();

    let mut transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(entity).unwrap();
    let velocity: ComponentRef<'_, Velocity> = context.world.get_entity_component::<Velocity>(entity).unwrap();

    transform.position.y += velocity.y * context.delta;
}
