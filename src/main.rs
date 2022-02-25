extern crate pancurses;
use dungeon_models::{Dungeon, Field};
use generator::bsp::BSPGenerator;
use generator::cellular::CellularAutomataGenerator;
use generator::dfs::DFSGenerator;
use generator::DungeonBuilder;
//use narrative::dummy::DummyNarrativeBuilder;
use narrative::longpath::LongPathNarrativeBuilder;
use narrative::NarrativeBuilder;

const WALLS_ID: i16 = 1;
const ENTER_ID: i16 = 2;
const WALLS_COLOR: i16 = pancurses::COLOR_GREEN;
const ENTER_COLOR: i16 = pancurses::COLOR_RED;

fn draw_dungeon(dungeon: &Dungeon) {
    let window = pancurses::initscr();
    pancurses::curs_set(0);
    pancurses::start_color();
    pancurses::init_pair(WALLS_ID, WALLS_COLOR, pancurses::COLOR_BLACK);
    pancurses::init_pair(ENTER_ID, ENTER_COLOR, pancurses::COLOR_BLACK);
    window.color_set(WALLS_ID);
    for y in 0..dungeon.size_y {
        for x in 0..dungeon.size_x {
            match dungeon.map[x][y] {
                Field::Wall => window.mvprintw(y as i32, x as i32, "#"),
                Field::Floor => window.mvprintw(y as i32, x as i32, "."),
                Field::Exit => {
                    window.color_set(ENTER_ID);
                    window.mvprintw(y as i32, x as i32, ">");
                    window.color_set(WALLS_ID)
                },
                Field::Enter => {
                    window.color_set(ENTER_ID);
                    window.mvprintw(y as i32, x as i32, "<");
                    window.color_set(WALLS_ID)
                },
                _ => window.mvprintw(y as i32, x as i32, "/"),
            };
        }
    }
    window.refresh();
    window.getch();
    pancurses::endwin();
}

fn main() {    
    let dfs_gen = DFSGenerator {
        size_x: 16,
        size_y: 16,
    };
    let mut dungeon = dfs_gen.build();
    //let narrative_builder = DummyNarrativeBuilder {};
    let narrative_builder = LongPathNarrativeBuilder {};
    narrative_builder.fill(&mut dungeon);
    draw_dungeon(&dungeon);

    let cellular_gen = CellularAutomataGenerator {
        size_x: 50,
        size_y: 50,
        iterations: 10,
        wall_probability: 0.5,
    };
    let mut dungeon = cellular_gen.build();
    //let narrative_builder = DummyNarrativeBuilder {};
    let narrative_builder = LongPathNarrativeBuilder {};
    narrative_builder.fill(&mut dungeon);
    draw_dungeon(&dungeon);

    let bsp_gen = BSPGenerator {
        size_x: 50,
        size_y: 50,
        split_size: 15,
        wall_size: 2,
    };
    let mut dungeon = bsp_gen.build();
    //let narrative_builder = DummyNarrativeBuilder {};
    let narrative_builder = LongPathNarrativeBuilder {};
    narrative_builder.fill(&mut dungeon);
    draw_dungeon(&dungeon);
}
