use std::option::{Option, Some, None};
enum Nation {
    Austria,
    England,
    France,
    Germany,
    Italy,
    Russia,
    Turkey
}
#[deriving(ToStr)]
enum TerritoryType {
    Landlocked,
    Coastal,
    Water
}
enum UnitType {
    Army,
    Fleet
}
struct Unit {
    unit_type: UnitType,
    owner: Nation
}
struct Province {
    name: ~str,
    power: Option<Nation>,
    terrain: TerritoryType,
    occupier: Option<Unit>,
    is_supply_center: bool
}
pub fn world() {
    let province =
        Province {
            name: ~"Marseilles", power: Some(France),
            terrain: Coastal, occupier: None,
            is_supply_center: true };
    println!("{} is a {} region",
             province.name, province.terrain.to_str());
}
