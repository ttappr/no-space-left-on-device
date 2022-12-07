#![allow(dead_code)]

/// This module defines the FSObject trait and the FSDir and FSFile structs.
/// The objects implemented in this module are used to represent the file 
/// system.

use std::cell::{RefCell, RefMut, Ref};
use std::collections::BTreeMap;
use std::fmt::{Formatter, Debug};
use std::rc::Rc;

macro_rules! pwrap { ($e:expr) => { Rc::new(RefCell::new($e)) } }

/// A trait for objects in the file system.
/// 
pub trait FSObject {
    /// Return the name of the object.
    fn name(&self) -> String;
    /// Return the size of the object.
    fn size(&self) -> usize;
    /// Return the parent of the object.
    fn parent(&self) -> Option<FSDir>;
}

/// Holds a file or dir in the file system.
/// 
#[derive(Debug)]
enum FSDirOrFile {
    File(FSFile),
    Dir(FSDir),
}

/// The impl data for a FS directory. This is wrapped in a RefCell so that
/// we can have multiple references to the same directory.
/// 
struct FSDirImpl  { 
    name     : String, 
    size     : usize,
    children : BTreeMap<String, FSDirOrFile>, 
    parent   : Option<FSDir>,
}
impl Debug for FSDirImpl {
    /// This is a custom debug impl to avoid infinite recursion.
    /// 
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FSDirImpl")
            .field("name", &self.name)
            .field("size", &self.size)
            .field("children", &self.children)
            //.field("parent", "skipped..")
            .finish()
    }
}

/// A directory in the FS file system. Internally this is a RefCell so that
/// we can have multiple references to the same directory.
/// 
#[derive(Debug, Clone)]
pub struct FSDir {
    pimpl: Rc<RefCell<FSDirImpl>>,
}
impl FSDir {
    /// Create a new directory with the given name.
    pub fn new(name: String) -> Self {
        Self {
            pimpl: pwrap!(FSDirImpl { 
                name, 
                children: BTreeMap::new(), 
                parent: None, 
                size: 0 
            }),
        }
    }
    /// Returns true if the directory contains a child with the given name.
    pub fn contains(&self, name: &str) -> bool {
        self.get_ref().children.contains_key(name)
    }
    /// Adds a directory to this directory.
    pub fn add_dir(&self, dir: FSDir)  {
        dir.set_parent(self.clone());
        self.incr_size(dir.size());
        self.get_mut().children.insert(dir.name(), FSDirOrFile::Dir(dir));
    }
    /// Adds a file to this directory.
    pub fn add_file(&self, file: FSFile) {
        file.set_parent(self.clone());
        self.incr_size(file.size());
        self.get_mut().children.insert(file.name(), FSDirOrFile::File(file));
    }
    /// Returns the directory object with the given name.
    pub fn get_dir(&self, name: &str) -> Option<FSDir> {
        match self.get_ref().children.get(name) {
            Some(FSDirOrFile::Dir(dir)) => Some(dir.clone()),
            _ => None,
        }
    }
    /// Returns a list of all the files in this directory that match the 
    /// predicate.
    /// 
    pub fn find_dirs_by<F>(&self, pred: &F)  -> Vec<FSDir> 
    where 
        F: Fn(&FSDir) -> bool
    {
        let mut dirs = vec![];
        for (_, child) in self.get_ref().children.iter() {
            if let FSDirOrFile::Dir(dir) = child {
                if pred(dir) {
                    dirs.push(dir.clone());
                }
            }
        }
        dirs
    }
    /// Returns a list of all the files in this directory that match the
    /// predicate. The directory structure is traversed recursively.
    /// 
    pub fn find_dirs_recurs_by<F>(&self, pred: &F) -> Vec<FSDir> 
    where
        F: Fn(&FSDir) -> bool
    {
        let mut dirs = vec![];
        for (_, child) in self.get_ref().children.iter() {
            if let FSDirOrFile::Dir(dir) = child {
                if pred(dir) {
                    dirs.push(dir.clone());
                }
                dirs.extend(dir.find_dirs_recurs_by(pred));
            }
        }
        dirs
    }
    /// Returns a mutable reference to the internal RefCell.
    fn get_mut(&self) -> RefMut<FSDirImpl> {
        self.pimpl.borrow_mut()
    }
    /// Returns a reference to the internal RefCell.
    fn get_ref(&self) -> Ref<FSDirImpl> {
        self.pimpl.borrow()
    }
    /// Sets the parent of this directory.
    fn set_parent(&self, parent: FSDir) {
        self.get_mut().parent = Some(parent);
    }
    /// Increments the size of this directory and all its parents.
    /// 
    fn incr_size(&self, size: usize) {
        let mut pimpl = self.get_mut();
        pimpl.size += size;
        if let Some(parent) = pimpl.parent.clone() {
            parent.incr_size(size);
        }
    }
}
impl FSObject for FSDir {
    fn name(&self) -> String {
        self.get_ref().name.clone()
    }
    fn size(&self) -> usize {
        self.get_ref().size
    }
    fn parent(&self) -> Option<FSDir> {
        self.get_ref().parent.clone()
    }
}

/// The impl data for a FS file.
/// 
struct FSFileImpl { 
    name: String, 
    size: usize,
    parent: Option<FSDir>,
}
impl Debug for FSFileImpl {
    /// This is a custom debug impl to avoid infinite recursion.
    /// 
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FSFileImpl")
            .field("name", &self.name)
            .field("size", &self.size)
            //.field("parent", "skipped..")
            .finish()
    }
}

/// A file in the FS file system.
/// 
#[derive(Debug, Clone)]
pub struct FSFile {
    pimpl: Rc<RefCell<FSFileImpl>>,
}
impl FSFile {
    /// Creates a new file.
    pub fn new(name: String, size: usize) -> Self {
        Self {
            pimpl: pwrap!(FSFileImpl { name, size, parent: None }),
        }
    }
    /// Returns a mutable reference to the internal RefCell.
    fn get_mut(&self) -> RefMut<FSFileImpl> {
        self.pimpl.borrow_mut()
    }
    /// Returns a reference to the internal RefCell.
    fn get_ref(&self) -> Ref<FSFileImpl> {
        self.pimpl.borrow()
    }
    /// Sets the parent of this file.
    fn set_parent(&self, parent: FSDir) {
        self.get_mut().parent = Some(parent);
    }
}
impl FSObject for FSFile {
    fn name(&self) -> String {
        self.pimpl.borrow().name.clone()
    }
    fn size(&self) -> usize {
        self.pimpl.borrow().size
    }
    fn parent(&self) -> Option<FSDir> {
        self.pimpl.borrow().parent.clone()
    }
}
