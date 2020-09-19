
pub trait Weapon{

    fn mount (&self);
}

pub struct BasicWeapon;

impl Weapon for BasicWeapon {

    fn mount(&self) {
        println!("This is a basic weapon!");
    }
}

pub struct Target<'a>{
     pub(crate) weapon: &'a Weapon,
}

impl<'a> Weapon for Target<'a> {
    fn mount(&self) {
        self.weapon.mount();
        println!("Add a Target in the weapon!");
    }
}

pub struct Silencer<'a>{
    pub(crate) weapon: &'a Weapon
}

impl<'a> Weapon for Silencer<'a>{
    fn mount(&self) {
        self.weapon.mount();
        println!("Add a Silencer in the weapon!");
    }
}

