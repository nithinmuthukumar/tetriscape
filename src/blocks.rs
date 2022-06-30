use bevy::prelude::*;
use lazy_static::*;
use rand::prelude::*;


#[derive(Component,Copy,Clone)]
pub struct Tetromino{
    blocks: [Vec2; 4]
}
impl Tetromino{
    fn rand()->Tetromino{
        let mut rng = thread_rng();
        let r = rng.gen_range(0..=6);
        return TETROMINOS[r][0];
    }
}



#[derive(Component,Default)]
struct Block;

#[derive(Bundle,Default)]
struct BlockBundle{
    block:Block,
    #[bundle]
    sprite: SpriteBundle,
    //add physics object
}

pub fn spawn_tetromino(cmd: &mut Commands,assets: &Res<AssetServer>, init_pos: Vec2){
    cmd.spawn()
        .insert(Tetromino::rand())
        .insert(Transform{
            translation:init_pos.extend(0.),
            ..default()
        })
    .with_children(|parent| {
        for n in 0..4{
            parent.spawn_bundle(BlockBundle::default());
        }
    });
}






lazy_static! {
    pub static ref TETROMINOS: Vec<Vec<Tetromino>> = vec![
        //O:
        vec![Tetromino{blocks:[Vec2::new(1., 1.), Vec2::new(1., 2.), Vec2::new(2., 1.), Vec2::new(2., 2.)]}],
        //I:
        vec![
            Tetromino{blocks:[Vec2::new(0., 1.), Vec2::new(1., 1.), Vec2::new(2., 1.), Vec2::new(3., 1.)]},
            Tetromino{blocks:[Vec2::new(2., 0.), Vec2::new(2., 1.), Vec2::new(2., 2.), Vec2::new(2., 3.)]}
        ],
        //J:
        vec![
            Tetromino{blocks:[Vec2::new(0., 1.), Vec2::new(1., 1.), Vec2::new(2., 1.), Vec2::new(2., 0.)]},
            Tetromino{blocks:[Vec2::new(1., 0.), Vec2::new(1., 1.), Vec2::new(1., 2.), Vec2::new(0., 0.)]},
            Tetromino{blocks:[Vec2::new(0., 1.), Vec2::new(1., 1.), Vec2::new(2., 1.), Vec2::new(0., 2.)]},
            Tetromino{blocks:[Vec2::new(1., 0.), Vec2::new(1., 1.), Vec2::new(1., 2.), Vec2::new(2., 2.)]},
        ],
        //L:
        vec![
            Tetromino{blocks:[Vec2::new(0., 1.), Vec2::new(1., 1.), Vec2::new(2., 1.), Vec2::new(0., 0.)]},
            Tetromino{blocks:[Vec2::new(1., 0.), Vec2::new(1., 1.), Vec2::new(1., 2.), Vec2::new(0., 2.)]},
            Tetromino{blocks:[Vec2::new(0., 1.), Vec2::new(1., 1.), Vec2::new(2., 1.), Vec2::new(2., 2.)]},
            Tetromino{blocks:[Vec2::new(1., 0.), Vec2::new(1., 1.), Vec2::new(1., 2.), Vec2::new(2., 0.)]},
        ],
        //S:
        vec![
            Tetromino{blocks:[Vec2::new(0., 0.), Vec2::new(1., 0.), Vec2::new(1., 1.), Vec2::new(2., 1.)]},
            Tetromino{blocks:[Vec2::new(1., 2.), Vec2::new(1., 1.), Vec2::new(2., 1.), Vec2::new(2., 0.)]},
        ],
        //Z:
        vec![
            Tetromino{blocks:[Vec2::new(0., 1.), Vec2::new(1., 1.), Vec2::new(1., 0.), Vec2::new(2., 0.)]},
            Tetromino{blocks:[Vec2::new(2., 2.), Vec2::new(2., 1.), Vec2::new(1., 1.), Vec2::new(1., 0.)]},
        ],
        //T:
        vec![
            Tetromino{blocks:[Vec2::new(0., 1.), Vec2::new(1., 1.), Vec2::new(2., 1.), Vec2::new(1., 0.)]},
            Tetromino{blocks:[Vec2::new(1., 0.), Vec2::new(1., 1.), Vec2::new(1., 2.), Vec2::new(0., 1.)]},
            Tetromino{blocks:[Vec2::new(0., 1.), Vec2::new(1., 1.), Vec2::new(2., 1.), Vec2::new(1., 2.)]},
            Tetromino{blocks:[Vec2::new(1., 0.), Vec2::new(1., 1.), Vec2::new(1., 2.), Vec2::new(2., 1.)]},
        ],
    ];
}
