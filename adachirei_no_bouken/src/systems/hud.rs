use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
#[read_component(Damage)]
#[read_component(Defense)]
pub fn hud(ecs: &SubWorld) {
    //Health Bar
    let mut health_query = <&Health>::query().filter(component::<Player>()); // (1)
    let player_health = health_query
        .iter(ecs)
        .nth(0) // (2)
        .unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2); // (3)
    draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move."); // (4)
    draw_batch.bar_horizontal(
        // (5)
        Point::zero(),              // (6)
        SCREEN_WIDTH * 2,           // (7)
        player_health.current,      // (8)
        player_health.max,          // (9)
        ColorPair::new(RED, BLACK), // (10)
    );
    draw_batch.print_color_centered(
        0,
        format!(
            " Health: {} / {} ",
            player_health.current, player_health.max
        ),
        ColorPair::new(WHITE, RED),
    );

    //Display the current level, and inventory, attack, and defense
    let (player, map_level) = <(Entity, &Player)>::query()
        .iter(ecs)
        .map(|(entity, player)| (entity, player.map_level))
        .nth(0)
        .unwrap();
    //print map_level
    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH * 2, 1),
        format!("Dungeon Level: {}", map_level),
        ColorPair::new(YELLOW, BLACK),
    );
    //print attack and defense
    let base_damage = if let Ok(v) = ecs.entry_ref(*player) {
        if let Ok(dmg) = v.get_component::<Damage>() {
            dmg.0
        } else {
            0
        }
    } else {
        0
    };
    let weapon_damage: i32 = <(&Carried, &Damage)>::query()
        .iter(ecs)
        .filter(|(carried, _)| carried.0 == *player)
        .map(|(_, dmg)| dmg.0)
        .sum();
    let base_defense = if let Ok(v) = ecs.entry_ref(*player) {
        if let Ok(defense) = v.get_component::<Defense>() {
            defense.0
        } else {
            0
        }
    } else {
        0
    };
    let armor_defense: i32 = <(&Carried, &Defense)>::query()
        .iter(ecs)
        .filter(|(carried, _)| carried.0 == *player)
        .map(|(_, defense)| defense.0)
        .sum();
    let final_attack = base_damage + weapon_damage;
    let final_defense = base_defense + armor_defense;
    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH * 2, 3),
        format!("Attack: {}", final_attack),
        ColorPair::new(YELLOW, BLACK),
    );
    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH * 2, 4),
        format!("Defense: {}", final_defense),
        ColorPair::new(YELLOW, BLACK),
    );

    let mut item_query = <(&Name, &Carried)>::query().filter(component::<Item>());
    let mut y = 3;
    item_query
        .iter(ecs)
        .filter(|(_, carried)| carried.0 == *player)
        .for_each(|(name, _)| {
            draw_batch.print(Point::new(3, y), format!("{} : {}", y - 2, &name.0));
            y += 1;
        });
    if y > 3 {
        draw_batch.print_color(
            Point::new(3, 2),
            "Items carried",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    draw_batch.submit(10000).expect("Batch error");
}
