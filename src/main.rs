extern crate pancurses;
use dungeon_models::{Dungeon, Field};
use generator::bsp::BSPGenerator;
use generator::dfs::DFSGenerator;
use generator::DungeonBuilder;
use narrative::dummy::DummyNarrativeBuilder;
use narrative::NarrativeBuilder;

fn draw_dungeon(dungeon: &Dungeon) {
    let window = pancurses::initscr();
    pancurses::curs_set(0);
    for y in 0..dungeon.size_y {
        for x in 0..dungeon.size_x {
            match dungeon.map[x][y] {
                Field::Wall => window.mvprintw(y as i32, x as i32, "#"),
                Field::Floor => window.mvprintw(y as i32, x as i32, "."),
                Field::Exit => window.mvprintw(y as i32, x as i32, ">"),
                Field::Enter => window.mvprintw(y as i32, x as i32, "<"),
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
        size_x: 10,
        size_y: 10,
    };
    let mut dungeon = dfs_gen.build();
    let narrative_builder = DummyNarrativeBuilder {};
    narrative_builder.fill(&mut dungeon);
    draw_dungeon(&dungeon);
    let bsp_gen = BSPGenerator {
        size_x: 30,
        size_y: 30,
        split_size: 15,
        wall_size: 2,
    };
    let mut dungeon = bsp_gen.build();
    let narrative_builder = DummyNarrativeBuilder {};
    narrative_builder.fill(&mut dungeon);
    draw_dungeon(&dungeon);
}
