#[derive(Debug)]
enum Event {
    Arrived(i32),
    DoorOpened,
    DoorClosed,
    CallPressed(i32, Direction),
    FloorButtonPressed(i32),
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

fn elevator_arrived(floor: i32) -> Event {
    Event::Arrived(floor)
}

fn door_opened() -> Event {
    Event::DoorOpened
}

fn door_closed() -> Event {
    Event::DoorClosed
}

fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::CallPressed(floor, dir)
}

fn floor_button_pressed(floor: i32) -> Event {
    Event::FloorButtonPressed(floor)
}

fn main() {
    println!("Симуляция лифта");

    println!("{:?}", lobby_call_button_pressed(0, Direction::Up));
    println!("{:?}", elevator_arrived(0));
    println!("{:?}", door_opened());
    println!("{:?}", floor_button_pressed(3));
    println!("{:?}", door_closed());
    println!("{:?}", elevator_arrived(3));
}