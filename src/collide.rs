pub mod collide {
    use crate::game_object::game_object::Renderable;

    pub fn is_colliding<A, B>(a: &A, b: &B) -> bool
    where
        A: Renderable,
        B: Renderable,
    {
        let a_pos = a.get_position();
        let a_x = a_pos.x;
        let a_y = a_pos.y;

        let b_pos = b.get_position();
        let b_x = b_pos.x;
        let b_y = b_pos.y;

        (a_x >= b_x && a_x <= b_x + b_pos.w) && (a_y >= b_y && a_y <= b_y + b_pos.h)
    }
}
