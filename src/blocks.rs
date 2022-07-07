use bevy::prelude::*;
use lazy_static::*;
use rand::prelude::*;
use super::assetloader::BlockSheet;
#[derive(Component,Clone)]
pub struct Tetromino{
    index:usize,
    rot_index:usize
}
impl Tetromino{
    fn rand()->Tetromino{
        let mut rng = thread_rng();
        let r = rng.gen_range(0..=6);
        return Tetromino{index:r,rot_index:0};
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
            .add_system(spawn_tetromino)
            .add_system(tetromino_controller)
            .add_system(position_blocks)
            ;
    }
}



#[derive(Component)]
pub struct Block;

pub fn level(mut spawn_event:EventWriter<SpawnTetrominoEvent>){
    spawn_event.send(SpawnTetrominoEvent{
        init_pos:Vec3::new(0.,0.,0.)
    });
}


pub fn spawn_tetromino(mut cmd: Commands,block_sheet: Res<BlockSheet>, mut event:EventReader<SpawnTetrominoEvent>){
    for ev in event.iter(){
        let tetro = Tetromino::rand();
        cmd.spawn()
            .insert(tetro.clone())
            .insert(Transform{
                translation:ev.init_pos,
                ..default()
            })
        .insert(GlobalTransform{
            translation:ev.init_pos,
            ..default()
        })
        .with_children(|parent| {
            for n in TETROMINOS[tetro.index][tetro.rot_index] {
                parent.spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: 7,
                        ..default()
                },
                transform: Transform{
                    translation:n.extend(0.)*16.,
                    scale:Vec3::new(0.25,0.25,0.25),
                    ..default()
                },
                texture_atlas: block_sheet.0.clone(),
                ..default()
                }).insert(Block);
            }


        }); 



    }
}

pub fn tetromino_controller(
    mut cmd: Commands,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Tetromino)>){
    for (mut transform,mut tetro) in query.iter_mut(){
        if input.just_pressed(KeyCode::W) {
            tetro.rot_index+=1;
            tetro.rot_index%=TETROMINOS[tetro.index].len();

        } else if input.just_pressed(KeyCode::S) {
            if tetro.rot_index==0{
                tetro.rot_index=TETROMINOS[tetro.index].len()-1;
            }else{
                tetro.rot_index-=1;
            }


        }
        if input.just_pressed(KeyCode::A) {
            transform.translation.x-=16.;

        } else if input.just_pressed(KeyCode::D) {
            transform.translation.x+=16.;
        }

    }

}
pub fn position_blocks(
    mut blocks: Query<(&mut Transform),With<Block>>,
    mut tetrominos:Query<(&mut Tetromino,&Children),Without<Block>>){
    for (tetro,children) in tetrominos.iter_mut(){
        for (i,child) in children.iter().enumerate(){
            if let Ok(mut transform) = blocks.get_mut(*child){
                
                

                transform.translation=TETROMINOS[tetro.index][tetro.rot_index][i].extend(0.)*16.;
            }

            



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
