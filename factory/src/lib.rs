trait CheckBox{
    fn show(&self) -> &'static str;
}
trait Button{
    fn show(&self) -> &'static str;
}

pub trait AbstractFactory {
    fn create_checkbox(&self) -> Box<dyn CheckBox>;
    fn create_button(&self) -> Box<dyn Button>;
}


struct WindowsButton;

struct WindowsCheckBox;

impl CheckBox for WindowsCheckBox {
    fn show(&self) -> &'static str {
        "Hello i am windows checkbox"
    }
}
impl Button for WindowsButton {
    fn show(&self) -> &'static str {
        "Hello i am windows button"
    }
}

struct MacOsButton;

struct MacOsCheckBox;

impl CheckBox for MacOsCheckBox {
    fn show(&self) -> &'static str {
        "Hello i am mac os checkbox"
    }
}
impl Button for MacOsButton {
    fn show(&self) -> &'static str {
        "Hello i am mac os button"
    }
}


struct WindowsFactory;


impl WindowsFactory {
    fn new() -> Self {
        WindowsFactory {}
    }
}

impl AbstractFactory for WindowsFactory {
    fn create_checkbox(&self) -> Box<dyn CheckBox> {
        Box::new(WindowsCheckBox {})
    }

    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton {})
    }
}


struct MacOsFactory;

impl MacOsFactory {
    fn new() -> Self {
        MacOsFactory {}
    }
}


impl AbstractFactory for MacOsFactory {
    fn create_checkbox(&self) -> Box<dyn CheckBox> {
        Box::new(MacOsCheckBox {})
    }

    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacOsButton {})
    }
}


pub fn get_factory() -> Box<dyn AbstractFactory> {
    let os = std::env::args().last();
    match os {
        Some(val) => {
            if val.to_lowercase() == "mac_os" {
                return Box::new(MacOsFactory::new())
            }
            else if val.to_lowercase() == "windows" {
                return Box::new(WindowsFactory::new())
            }
            else {
                panic!("No specified factory for this operating system");
            }
        }
        None => panic!("Specify operation system")
    }
}

pub fn client_code(factory: Box<dyn AbstractFactory>) {
    let button = factory.create_button();
    let checkbox = factory.create_checkbox();
    println!("{:?}", button.show());
    println!("{:?}", checkbox.show());
}
