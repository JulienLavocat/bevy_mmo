use bevy::app::App;

pub trait EventSet {
    fn apply(app: &mut App);
}

pub trait AddEventSet {
    fn add_event_set<E: EventSet>(&mut self) -> &mut Self;
}

pub trait SendEvent<T> {
    fn send(&mut self, event: T);
}

impl AddEventSet for App {
    fn add_event_set<E: EventSet>(&mut self) -> &mut Self {
        E::apply(self);
        self
    }
}

#[macro_export]
macro_rules! packets_set {
    ($enum_name: ident, $name: ident {}) => {
        compile_error!("cannot make an empty event set");
    };
    ($enum_name: ident, $handler_name: ident, $name: ident { $($event: ident),* $(,)?}) => {
        #[allow(non_snake_case)]
        #[derive(bevy::ecs::system::SystemParam)]
        pub struct $name<'w> {
            $(
                $event: bevy::prelude::EventWriter<'w, $event>,
            )*
        }

        impl<'a> EventSet for $name<'a> {
            fn apply(app: &mut bevy::prelude::App) {
                $(
                    app.add_event::<$event>();
                )*
            }
        }

        $(
            impl<'a> SendEvent<$event> for $name<'a> {
                fn send(&mut self, event: $event) {
                    self.$event.send(event);
                }
            }
        )*

        #[derive(Serialize, Deserialize, Debug)]
        pub enum $enum_name {
            $(
                $event($event),
            )*
        }

        pub fn $handler_name(packets: &mut $name, message: &bevy_renet::renet::Bytes) {
            let message: $enum_name = bincode::deserialize(message).unwrap();
            match message {
                $(
                    $enum_name::$event(packet) => {
                         bevy::log::trace!("matched {:?} packet", packet);
                        packets.send(packet);
                    }
                )*
            }
        }

    };
}
