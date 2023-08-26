use backend::Pitou;

#[derive(PartialEq)]
pub struct Selected {
    allitems: Vec<Pitou>,
    selected: Vec<bool>,
    len_selt: usize,
}

static mut SELECTED: Option<Selected> = None;

pub fn clear() {
    unsafe { SELECTED = None }
}

pub fn len() -> usize {
    unsafe {
        SELECTED
            .as_ref()
            .map(|selected| selected.len_selt)
            .unwrap_or_default()
    }
}

pub fn all() -> Option<impl Iterator<Item = &'static Pitou>> {
    if len() == 0 {
        return None;
    } else {
        unsafe {
            let Selected {
                allitems,
                selected,
                len_selt: _,
            } = SELECTED.as_ref().unwrap();
            let res = allitems
                .into_iter()
                .enumerate()
                .filter(|(idx, _)| selected[*idx])
                .map(|(_, p)| p);
            Some(res)
        }
    }
}
