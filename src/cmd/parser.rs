const DEFAULT_LISTEN_ADDRESS : &str = "0.0.0.0";
const DEFAULT_PORT : u32 = 8009;

pub struct CommandLineParser {
    args: Vec<String>
}
  
impl CommandLineParser {
    pub fn new(args: Vec<String>) -> Self {
      Self {
        args: args
      }
    }

    pub fn parse_flags(&self, flags: &str) -> super::opts::Flags{

      let mut enabled_flags = super::opts::Flags::new();

      if flags.split(" ").collect::<Vec<&str>>().contains(&"--debug-mode") {
        enabled_flags.add( super::opts::FlagType::DebugMode )
      }

      return enabled_flags;

    }

    pub fn parse(&mut self) -> super::opts::CommandLineOpts {
  
      let mut opts = super::opts::CommandLineOpts { 
        listen_address: DEFAULT_LISTEN_ADDRESS, 
        port: DEFAULT_PORT,
        flags: super::opts::Flags::new()
      };
  
      match self.args.get(1) {
        Some(listen_address) => {
          opts.listen_address = listen_address;
        }
        None => {}
      }
  
      match self.args.get(2) {
        Some(port) => {
          opts.port = port.parse().unwrap();
        }
        None => {}
      }

      match self.args.get(3) {
        Some(flags) => {
            opts.flags = self.parse_flags(flags);
        }
        None => {}
      }
  
      return opts;
  
    }
  }