use bevy::prelude::*;
use lazy_static::*;
use rand::prelude::*;
use super::assetloader::BlockSheet;
#[derive(Component,Clone)]
pub struct Tetromino{
    blocks: [Vec2; 4]
}
impl Tetromino{
    fn rand()->Tetromino{
        let mut rng = thread_rng();
        let r = rng.gen_range(0..=6);
        return Tetromino{blocks:TETROMINOS[r][0]};
    }
}

pub struct SpawnTetrominoEvent{
    init_pos: Vec3,
}
pub struct BlocksPlugin;
impl Plugin for BlocksPlugin{
    fn build(&self, app: &mut App) {
        
        app.add_event::<SpawnTetrominoEvent>()
            .add_startup_system(level)
            .add_system(spawn_tetromino);
    }
}



#[derive(Component,Default)]
struct Block;

pub fn level(mut spawn_event:EventWriter<SpawnTetrominoEvent>){
    spawn_event.send(SpawnTetrominoEvent{
        init_pos:Vec3::ZERO
    });
}


pub fn spawn_tetromino(mut cmd: Commands,block_sheet: Res<BlockSheet>, mut event:EventReader<SpawnTetrominoEvent>){
    for ev in event.iter(){
        let tetro = Tetromino::rand();
        let parent = cmd.spawn()
            .insert(tetro.clone())
            .insert(Transform{
                translation:ev.init_pos,
                scale: Vec3::new(1.,1.,1.),
                ..default()
        }).id();
        for n in tetro.blocks{
            let child = cmd.spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 7,
                    ..default()
                },
                texture_atlas: block_sheet.0.clone(),
                ..default()
            }).id();
            // cmd.entity(parent).push_children(&[child]);


            
        }
        


    }
}






lazy_static! {
    pub static ref TETROMINOS: Vec<Vec<[Vec2;4]>> = vec![
        //O:
        vec![[Vec2::new(1., 1.), Vec2::new(1., 2.), Vec2::new(2., 1.), Vec2::new(2., 2.)]],
        //I:
        vec![
            [Vec2::new(0., 1.), Vec2::new(1., 1.), Vec2::new(2., 1.), Vec2::new(3., 1.)],
            [Vec2::new(2., 0.), Vec2::new(2., 1.), Vec2::new(2., 2.), Vec2::new(2., 3.)]
        ],
        //J:
        vec![
            [Vec2::new(0., 1.), Vec2::new(1., 1.), Vec2::new(2., 1.), Vec2::new(2., 0.)],
            [Vec2::new(1., 0.), Vec2::new(1., 1.), Vec2::new(1., 2.), Vec2::new(0., 0.)],
            [Vec2::new(0., 1.), Vec2::new(1., 1.), Vec2::new(2., 1.), Vec2::new(0., 2.)],
            [Vec2::new(1., 0.), Vec2::new(1., 1.), Vec2::new(1., 2.), Vec2::new(2., 2.)],
        ],
        //L:
        vec![
            [Vec2::new(0., 1.), Vec2::new(1., 1.), Vec2::new(2., 1.), Vec2::new(0., 0.)],
            [Vec2::new(1., 0.), Vec2::new(1., 1.), Vec2::new(1., 2.), Vec2::new(0., 2.)],
            [Vec2::new(0., 1.), Vec2::new(1., 1.), Vec2::new(2., 1.), Vec2::new(2., 2.)],
            [Vec2::new(1., 0.), Vec2::new(1., 1.), Vec2::new(1., 2.), Vec2::new(2., 0.)],
        ],
        //S:
        vec![
            [Vec2::new(0., 0.), Vec2::new(1., 0.), Vec2::new(1., 1.), Vec2::new(2., 1.)],
            [Vec2::new(1., 2.), Vec2::new(1., 1.), Vec2::new(2., 1.), Vec2::new(2., 0.)],
        ],
        //Z:
        vec![
            [Vec2::new(0., 1.), Vec2::new(1., 1.), Vec2::new(1., 0.), Vec2::new(2., 0.)],
            [Vec2::new(2., 2.), Vec2::new(2., 1.), Vec2::new(1., 1.), Vec2::new(1., 0.)],
        ],
        //T:
        vec![
            [Vec2::new(0., 1.), Vec2::new(1., 1.), Vec2::new(2., 1.), Vec2::new(1., 0.)],
            [Vec2::new(1., 0.), Vec2::new(1., 1.), Vec2::new(1., 2.), Vec2::new(0., 1.)],
            [Vec2::new(0., 1.), Vec2::new(1., 1.), Vec2::new(2., 1.), Vec2::new(1., 2.)],
            [Vec2::new(1., 0.), Vec2::new(1., 1.), Vec2::new(1., 2.), Vec2::new(2., 1.)],
        ],
    ];
}
