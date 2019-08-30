use std::collections::HashMap;

use crate::common::*;
use crate::element::*;
use crate::store::*;

///Holds and owns all elements, the index to them and their declarations. The store serves as an abstraction used by Documents
#[derive(Default)]
pub struct ElementStore {
    elements: Vec<Option<Box<FoliaElement>>>, //heap-allocated
    index: HashMap<String,IntId>
}

impl Store<FoliaElement> for ElementStore {
    fn items_mut(&mut self) -> &mut Vec<Option<Box<FoliaElement>>> {
        &mut self.elements
    }
    fn index_mut(&mut self) -> &mut HashMap<String,IntId> {
        &mut self.index
    }

    fn items(&self) -> &Vec<Option<Box<FoliaElement>>> {
        &self.elements
    }
    fn index(&self) -> &HashMap<String,IntId> {
        &self.index
    }
}

impl ElementStore {
    ///Adds an element as a child on another, this is a higher-level function that/
    ///takes care of adding and attaching for you.
    pub fn add_to(&mut self, parent_intid: IntId, child: FoliaElement) -> IntId {
        let child_intid = self.add(child);
        self.attach(parent_intid, child_intid);
        child_intid
    }

    ///Adds the child element to the parent element, automatically takes care
    ///of removing the old parent (if any).
    pub fn attach(&mut self, parent_intid: IntId, child_intid: IntId) -> bool {
        //ensure the parent exists
        if !self.get(parent_intid).is_some() {
            return false;
        };

        let oldparent_intid = if let Some(child) = self.get_mut(child_intid) {
            //add the new parent and return the old parent
            let tmp = child.get_parent();
            child.set_parent(Some(parent_intid));
            tmp
        } else {
            //child does not exist
            return false;
        };

        if let Some(parent) = self.get_mut(parent_intid) {
            parent.push(DataType::Element(child_intid));
        }

        if let Some(oldparent_intid) = oldparent_intid {
            //detach child from the old parent
            if let Some(oldparent) = self.get_mut(oldparent_intid) {
                if let Some(index) = oldparent.index(&DataType::Element(child_intid)) {
                    oldparent.remove(index);
                }
            }
        }
        true
    }

    ///Removes the child from the parent, orphaning it, does NOT remove the element entirely
    pub fn detach(&mut self, child_intid: IntId) -> bool {
        let oldparent_intid = if let Some(child) = self.get_mut(child_intid) {
            //add the new parent and return the old parent
            let tmp = child.get_parent();
            child.set_parent(None);
            tmp
        } else {
            //child does not exist
            return false;
        };

        if let Some(oldparent_intid) = oldparent_intid {
            //detach child from the old parent
            if let Some(oldparent) = self.get_mut(oldparent_intid) {
                if let Some(index) = oldparent.index(&DataType::Element(child_intid)) {
                    oldparent.remove(index);
                }
            }
        }
        true
    }
}

