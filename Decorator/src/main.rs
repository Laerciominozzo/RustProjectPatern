
mod weapon;

use weapon::*;
fn main() {
    let basic_weapon = BasicWeapon;
    basic_weapon.mount();

    println!("Putting a Target: ==================================");

    let target_weapon = Target{ weapon: &basic_weapon};
    target_weapon.mount();

    println!("Putting a Silencer: ==================================");

    let silencer_weapon = Silencer{ weapon:  &target_weapon};
    silencer_weapon.mount();
}
