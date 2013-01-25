use amulet::ll;
use amulet::ll::Style;
use amulet::ll::Window;

use entity::Entity;
use entity::Action;
use entity::AttackAction;
use entity::MoveAction;
use entity::WaitAction;
use geometry::Offset;
use interface::Interface;
use world::Map;
use world::World;

struct TerminalInterface {
    main_window: @Window,
    status_window: @Window,
    message_window: @Window,
}
pub fn make_terminal_interface() -> Interface {
    let window = amulet::ll::init_screen();
    window.hide_cursor();

    // Create persistent status areas
    let status_window = amulet::ll::new_window(0, 0, 0, 80);
    let message_window = amulet::ll::new_window(0, 80, 24, 0);

    return TerminalInterface{
        main_window: window,
        status_window: status_window,
        message_window: message_window,
    } as Interface;
}
impl TerminalInterface: Interface {
    fn next_action(world: &World) -> Action {
        let map = world.map;

        // Keep grabbing input until there's an actionable keypress
        loop {
            match self.main_window.read_key() {
                // TODO unclear how to pass this upwards; may need more complex
                // return type, boo
                ll::Character('q') => fail,

                ll::Character('.') => return WaitAction{ actor: map.player } as Action,

                ll::SpecialKey(ll::KEY_UP) => return self.pick_direction_action(world, Offset{ dx: 0, dy: -1 }),
                ll::SpecialKey(ll::KEY_DOWN) => return self.pick_direction_action(world, Offset{ dx: 0, dy: 1 }),
                ll::SpecialKey(ll::KEY_LEFT) => return self.pick_direction_action(world, Offset{ dx: -1, dy: 0 }),
                ll::SpecialKey(ll::KEY_RIGHT) => return self.pick_direction_action(world, Offset{ dx: 1, dy: 0 }),

                // TODO this is not an action
                ll::Character(',') => {
                    let player_tile = map.player_tile();
                    if player_tile.items.len() > 0 {
                        map.player.contents = map.player.contents + player_tile.items;
                        player_tile.items = ~[];
                    }
                    else {
                        self.message("nothing here...\n");
                    }
                }
                _ => {},
            }
        }
    }

    fn message(s: &str) {
        self.message_window.print(fmt!("%s\n", s));
    }

    fn redraw(world: &World) {
        self.draw_map(world);
        self.draw_status(world);
        self.draw_messages(world);
    }

    fn end() {
        self.main_window.read_key();
        libc::exit(0);
    }
}
impl TerminalInterface {
    fn pick_direction_action(world: &World, direction: Offset) -> Action {
        let player = world.map.player;
        let maybe_tile = world.map.tile_relative(player, direction);
        match maybe_tile {
            Some(tile) => {
                match tile.creature {
                    Some(creature) => {
                        return AttackAction{ actor: player, target: creature } as Action;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        return MoveAction{ actor: player, offset: direction } as Action;
    }


    fn _draw_entity(window: &Window, entity: @Entity) {
        window.attrwrite(fmt!("%c", entity.proto.display), &entity.proto.style);
    }

    fn draw_map(world: &World) {
        let map = world.map;
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
                self._draw_entity(self.main_window, entity);
            }
        }

        self.main_window.repaint();
    }

    fn draw_status(world: &World) {
        let map = world.map;
        let statwin = self.status_window;

        statwin.clear();
        statwin.print(fmt!("⌛ %u", world.clock));

        statwin.mv(1, 0);
        statwin.print(fmt!("♥ "));
        let mut healthbar = ~"";
        str::reserve(&mut healthbar, map.player.health);
        for (copy map.player.health).times {
            str::push_char(&mut healthbar, '█');
        }
        statwin.attrwrite(healthbar, &Style().fg(2));
        //statwin.attrwrite("░" * (5 - map.player.health) as uint, &Style().fg(1));

        statwin.mv(2, 0);
        statwin.print("inventory: ");
        for uint::range(0, map.player.contents.len()) |i| {
            self._draw_entity(statwin, map.player.contents[i]);
        }

        let tile = map.player_tile();
        if tile.items.len() > 0 {
            statwin.mv(4, 0);
            statwin.print("you see here:");
            statwin.mv(5, 4);
            for uint::range(0, tile.items.len()) |_i| {
                statwin.print("an item");
            }
        }
        statwin.repaint();
    }

    fn draw_messages(_world: &World) {
        self.message_window.repaint();
    }
}
