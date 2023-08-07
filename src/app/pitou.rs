use backend::Pitou;
use std::rc::Rc;
use yew::Properties;

#[derive(Clone)]
pub struct File {
    pitou: Rc<Pitou>,
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.pitou() == other.pitou()
    }
}

impl From<Pitou> for File {
    fn from(pitou: Pitou) -> Self {
        let pitou = Rc::new(pitou);
        File { pitou }
    }
}

impl File {
    pub fn pitou(&self) -> &Pitou {
        &self.pitou
    }
}

impl std::ops::Deref for File {
    type Target = Pitou;
    fn deref(&self) -> &Self::Target {
        self.pitou()
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct PitouProps {
    pub pitou: Pitou,
}
