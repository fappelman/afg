pub trait Result {
    fn result(&self) -> String;
}

pub trait Instantiate {
    fn instantiate(&self) -> String;
}

pub trait Declaration {
    fn declaration(&self) -> String;
}
