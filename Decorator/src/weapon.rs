
pub trait Weapon{

    fn mount (&self) -> String;

}


pub struct BasicWeapon;

impl Weapon for BasicWeapon {

    fn mount(&self) -> String {
        "This is a basic weapon".to_string()
    }
}

pub struct Target<'a>{
     pub(crate) weapon: &'a Weapon,
}

impl<'a> Weapon for Target<'a> {
    fn mount(&self) -> String{
        self.weapon.mount() + ", with a target"
    }
}

pub struct Silencer<'a>{
    pub(crate) weapon: &'a Weapon
}

impl<'a> Weapon for Silencer<'a>{
    fn mount(&self) -> String{
        self.weapon.mount() + ", with a silencer"
    }
}

