use amulet::ll::Style;
use amulet::ll::Window;

use entity::Entity;
use world::Map;
use world::Game;

struct TerminalDisplay {
    game: @Game,
    main_window: @Window,
    status_window: @Window,
    message_window: @Window,
}
pub fn TerminalDisplay(game: @Game) -> @TerminalDisplay {
    let window = amulet::ll::init_screen();
    window.hide_cursor();

    // Create persistent status areas
    let status_window = amulet::ll::new_window(0, 0, 0, game.map.width());
    let message_window = amulet::ll::new_window(0, 80, 24, 0);

    return @TerminalDisplay{
        game: game,
        main_window: window,
        status_window: status_window,
        message_window: message_window,
    };
}
impl TerminalDisplay {
    fn message(s: &str) {
        self.message_window.print(fmt!("%s\n", s));
    }

    fn update() {
        self.update_map();
        self.update_status();
        self.update_messages();
    }

    fn draw_entity(window: &Window, entity: @Entity) {
        window.attrprint(fmt!("%c", entity.proto.display), entity.proto.style);
    }

    fn update_map() {
        let map = self.game.map;
        for uint::range(0, map.width()) |x| {
            for uint::range(0, map.height()) |y| {
                let tile = map.grid[x][y];
                self.main_window.mv(y, x);
                let entity = match tile.creature {
                    Some(creature) => creature,
                    None => {
                        if tile.items.len() > 0 {
                            tile.items[0]
                        }
                        else {
                            tile.architecture
                        }
                    }
                };
                self.draw_entity(self.main_window, entity);
            }
        }

        self.main_window.repaint();
    }

    fn update_status() {
        let map = self.game.map;
        let statwin = self.status_window;

        statwin.clear();
        statwin.print(fmt!("⌛ %u", map.clock));

        statwin.mv(1, 0);
        statwin.print(fmt!("♥ "));
        let mut healthbar = ~"";
        str::reserve(&mut healthbar, map.player.health);
        for (copy map.player.health).times {
            str::push_char(&mut healthbar, '█');
        }
        statwin.attrprint(healthbar, Style().fg(2));
        //statwin.attrprint("░" * (5 - map.player.health) as uint, Style().fg(1));

        statwin.mv(2, 0);
        statwin.print("inventory: ");
        for uint::range(0, map.player.contents.len()) |i| {
            self.draw_entity(statwin, map.player.contents[i]);
        }

        let tile = map.player_tile();
        if tile.items.len() > 0 {
            statwin.mv(4, 0);
            statwin.print("you see here:");
            statwin.mv(5, 4);
            for uint::range(0, tile.items.len()) |i| {
                statwin.print("an item");
            }
        }
        statwin.repaint();
    }

    fn update_messages() {
        self.message_window.repaint();
    }
}
