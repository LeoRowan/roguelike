use super::{constants::*, entity::Entity, map::Point, Game};
use std::cmp;

pub fn ai_take_turn(monster_id: usize, game: &mut Game) {
    let Point { x, y } = game.state.entities[monster_id].position;
    let can_attack_player = {
        let player = &game.state.entities[PLAYER];
        let monster = &game.state.entities[monster_id];
        let distance = monster.position.distance_to(player.position);

        distance < 2.0 && player.fighter.map_or(false, |f| f.hp > 0)
    };

    match (game.tcod.fov.is_in_fov(x, y), can_attack_player) {
        (true, false) => {
            let player_position = game.state.entities[PLAYER].position;
            Entity::move_towards(monster_id, player_position, &mut game.state);
        }
        (true, true) => {
            let (player, monster) = mut_two(PLAYER, monster_id, &mut game.state.entities);
            monster.attack(player);
        }
        _ => (),
    }
}

pub fn mut_two<T>(first_idx: usize, second_idx: usize, items: &mut [T]) -> (&mut T, &mut T) {
    assert!(first_idx != second_idx);
    let split_idx = cmp::max(first_idx, second_idx);
    let (slice_a, slice_b) = items.split_at_mut(split_idx);

    if first_idx < second_idx {
        (&mut slice_a[first_idx], &mut slice_b[0])
    } else {
        (&mut slice_b[0], &mut slice_a[first_idx])
    }
}
