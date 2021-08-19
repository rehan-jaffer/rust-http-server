pub struct CommandLineOpts<'a> {
    pub listen_address: &'a str,
    pub port: u32,
    pub flags: Flags
}

pub struct Flags {
    flags: Vec<Flag>
}

impl Flags {

    pub fn new() -> Self {
        return Self {
            flags: vec![]
        }
    }

    pub fn list(&self) -> Vec<FlagType> {
        return self.flags.clone()
          .into_iter()
          .map(|flag| flag.flag_type )
          .collect::<Vec<FlagType>>();
    }

    pub fn add(&mut self, flag: FlagType) -> () {
      self.flags.push( Flag { flag_type: flag } );
    }

    pub fn contains(&self, flag: FlagType) -> bool {
        return self.flags.clone()
          .into_iter()
          .map(|flag| flag.flag_type)
          .collect::<Vec<FlagType>>()
          .contains(&flag);
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum FlagType {
    DebugMode
}

#[derive(Clone)]
pub struct Flag {
  pub flag_type: FlagType
}
