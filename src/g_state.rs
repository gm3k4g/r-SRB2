// the current state of the game
bitflags! {
	#[derive(Default)]
	pub struct GameStateT: u8 {
		const GS_NULL = 0; // At beginning

		// Fadeable Gamestates
		const GS_LEVEL = 1; //Playing in a level
		const GS_INTERMISSION = 2; // Intermission screen
		const GS_CONTINUING = 3; //Continue screen

		const GS_TITLESCREEN = 4; //Title screen
		const GS_TIMEATTACK = 5; //Time attack menu

		const GS_CREDITS = 6; //Credits sequence
		const GS_EVALUATION = 7; //Evaluation at the end of a game
		const GS_GAMEEND = 8; //Game End Sequence -- 'did you get all those chaos emeralds?'

		// HardCoded fades or other fading methods
		const GS_INTRO = 9; // Introduction
		const GD_ENDING = 10; //currently shared between bad and good endings
		const GD_CUTSCENE = 11; //Custom cutscene

		// Not fadeable
		const GS_DEDICATEDSERVER = 12; //new state for dedicated server
		const GS_WAITINGPLAYERS = 13; // waiting for players in a net game
	}
}

/*
impl Default for GameStateT {
	fn default() -> Self {
		Default::default()
	}
}*/

bitflags! {
	#[derive(Default)]
	pub struct GameActionT: u32 {
		const GA_NOTHING 	= 0;
        const GA_COMPLETED 	= 1;
        const GA_WORLD_DONE = 2;
        const GA_START_CONT = 3;
        const GA_CONTINUED 	= 4; 
	}
}


/*
impl Default for Game {
	fn default() -> Self {
	 ... 
	}
}
*/