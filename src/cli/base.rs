use crate::client::BaseClient;

pub trait SubCommandTrait {
    fn process(&self, client: &BaseClient);
}
