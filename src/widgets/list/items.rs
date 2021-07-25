use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use super::Item;

pub type Array = Rc<RefCell<Vec<Item>>>;

pub struct Items {
    items: Rc<RefCell<Vec<Item>>>,
}

impl Items {
    pub fn default() -> Self {
        Items {
            items: Array::default(),
        }
    }

    fn items(&self) -> Ref<Vec<Item>> {
        self.items.borrow()
    }

    fn items_mut(&self) -> RefMut<Vec<Item>> {
        self.items.borrow_mut()
    }

    pub fn len(&self) -> usize {
        self.items().len()
    }

    pub fn index(&self, index: usize) -> Ref<Item> {
        Ref::map(self.items(), |items| &items[index])
    }

    pub fn index_mut(&self, index: usize) -> RefMut<Item> {
        RefMut::map(self.items_mut(), |items| &mut items[index])
    }

    pub fn push(&self, item: Item) {
        self.items_mut().push(item);
    }

    pub fn select(&self, idx: usize) {
        if idx > 0 {
            let idx = idx - 1;
            for i in 0..self.len() {
                if i == idx {
                    self.index_mut(i).select();
                } else if self.index(i).selected() {
                    self.index_mut(i).unselect();
                }
            }
        } else {
            for i in 0..self.len() {
                self.index_mut(i).unselect();
            }
        }
    }

    pub fn remove(&self, index: usize) {
        self.items_mut().remove(index);
    }
}

impl Clone for Items {
    fn clone(&self) -> Self {
        Items {
            items: Rc::clone(&self.items),
        }
    }
}
