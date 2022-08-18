use ggez::{Context, GameError, GameResult};

use lang_vm::{
    bullet::{BulletColor, BulletType},
    VM,
};

use super::{bullet::BulletState, bullet_codes::BulletCodeMap, SceneDrawable};

pub struct Player {
    pub state: BulletState,
    pub vm: VM,
}

impl Player {
    pub fn new(code_map: &BulletCodeMap) -> Self {
        let state = BulletState::new(200.0, 400.0, BulletType::Player, BulletColor::White);
        let mut vm = VM::new();

        if let Some((code, memory)) = code_map.get("player") {
            eprintln!("VM code = {:?}", code);
            vm.set_code(code.clone());
            vm.set_memory(memory.clone());
        }

        Self { state, vm }
    }

    pub fn update(&mut self) -> GameResult<()> {
        if let Err(err) = self.vm.run(&mut self.state) {
            return Err(GameError::CustomError(format!("error = {:?}", err)));
        }

        Ok(())
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        if let Err(err) = self.state.draw(ctx) {
            return Err(GameError::CustomError(format!("error = {:?}", err)));
        }

        Ok(())
    }
}