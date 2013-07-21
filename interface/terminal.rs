use std::libc;
use std::uint;

use amulet;
use amulet::ll;
use amulet::ll::Style;
use amulet::canvas::Canvas;

use entity;
use entity::Entity;
use entity::Action;
use entity::AttackAction;
use entity::MoveAction;
use entity::WaitAction;
use geometry::Offset;
use interface::Interface;
use world::Map;
use world::World;

struct TerminalInterface<'self> {
    term: ~amulet::ll::Terminal,
    main_window: Canvas<'self>,
    status_window: Canvas<'self>,
    message_window: Canvas<'self>,
}
// XXX this should really return an Interface, but it cannot due
// to Rust bug #3794, which basically prevents me from ever
// borrowing the resulting ~Interface
pub fn make_terminal_interface() -> TerminalInterface {
    let term = ~amulet::ll::Terminal();
    let main_window = term.enter_fullscreen();

    // Create persistent status areas
    let status_window = main_window.spawn(0, 80, 0, 0);
    let message_window = main_window.spawn(24, 0, 0, 80);

    return TerminalInterface{
        term: term,
        main_window: main_window,
        status_window: status_window,
        message_window: message_window,
    };// as ~Interface;
}
impl<'self> Interface for TerminalInterface<'self> {
    fn next_action(&self, world: &World) -> ~Action:'static {
        let map = world.map;

        // Keep grabbing input until there's an actionable keypress
        loop {
            match self.main_window.read_key() {
                // TODO unclear how to pass this upwards; may need more complex
                // return type, boo
                ll::Character('q') => fail!(~"todo"),

                ll::Character('.') => return ~WaitAction{ actor: map.player } as ~Action:'static,

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

    fn message(&self, s: &str) {
        self.message_window.write(fmt!("%s\n", s));
    }

    fn redraw(&self, world: &World) {
        self.draw_map(world);
        self.draw_status(world);
        self.draw_messages(world);
    }

    fn end(&self) {
        self.main_window.read_key();
        // TODO this probably should (a) do more stuff and (b) let the ending
        // bubble up to the top instead of calling exit here
        unsafe {
            libc::exit(0);
        }
    }
}
impl<'self> TerminalInterface<'self> {
    fn pick_direction_action(&self, world: &World, direction: Offset) -> ~Action:'static {
        let player = world.map.player;
        let maybe_tile = world.map.tile_relative(player, direction);
        match maybe_tile {
            Some(tile) => {
                match tile.creature {
                    Some(creature) => {
                        return ~AttackAction{ actor: player, target: creature } as ~Action:'static;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        return ~MoveAction{ actor: player, offset: direction } as ~Action:'static;
    }


    fn _draw_entity(&self, window: &mut Canvas, entity: @mut Entity) {
        window.attrwrite(fmt!("%c", entity.proto.display), entity.proto.style);
    }

    fn draw_map(&mut self, world: &World) {
        let map = world.map;
        for uint::range(0, map.width()) |x| {
            for uint::range(0, map.height()) |y| {
                let tile = map.grid[x][y];
                self.main_window.move(y, x);
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
                self._draw_entity(&self.main_window, entity);
            }
        }

        self.main_window.repaint();
    }

    fn draw_status(&mut self, world: &World) {
        let map = world.map;
        let mut statwin = self.status_window;

        statwin.clear();
        statwin.write(fmt!("⌛ %u", world.clock));

        statwin.move(1, 0);
        statwin.write(fmt!("♥ "));
        let mut healthbar = ~"";
        healthbar.reserve(map.player.health);
        for (copy map.player.health).times {
            healthbar.push_char('█');
        }
        statwin.attrwrite(healthbar, Style().fg(2));
        //statwin.attrwrite("░" * (5 - map.player.health) as uint, Style().fg(1));

        statwin.move(2, 0);
        statwin.write("inventory: ");
        for uint::range(0, map.player.contents.len()) |i| {
            self._draw_entity(&statwin, map.player.contents[i]);
        }

        let tile = map.player_tile();
        if tile.items.len() > 0 {
            statwin.move(4, 0);
            statwin.write("you see here:");
            statwin.move(5, 4);
            for uint::range(0, tile.items.len()) |_i| {
                statwin.write("an item");
            }
        }
        statwin.repaint();
    }

    fn draw_messages(&self, _world: &World) {
        self.message_window.repaint();
    }
}
