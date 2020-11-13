use crate::bindings::*;

use crate::term::TermRefs;

//WARNING: When Drop is called, handle is dropped first (as it should) because it is the first field.
//Changing the field order can make this structure does not deinitialize correctly.


use std::cell::Cell;
use std::rc::Rc;
/**
Represent a prolog foreign frame. Frame allow to create term allocation and destroy all of them when deinitialized.
*/
pub struct Frame
{
    handle: FidT,
    parent: Option<Rc<Frame>>,
    have_child: Cell<bool>

}
impl Frame
{
    pub fn new(parent: Option<Rc<Frame>>)->Rc<Self>
    {
        Rc::new(Self
        {
            handle: open_foreign_frame(),
            parent: parent,
            have_child: Cell::new(false),
        })
    }

    pub fn nest(self: &Rc<Self>)->Option<Rc<Frame>>
    {
        if !self.have_child.get()
        {
            self.have_child.set(true);
            let frame = Self::new(Some(self.clone()));
            Some(frame)
        }
        else {None}
    }

    pub fn create_term_refs(self: &Rc<Self>,count: usize)->Option<TermRefs>
    {
        if !self.have_child.get()
        {
            Some(TermRefs::new(self.clone(),count))
        }
        else {None}
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        discard_foreign_frame(self.handle);
        match &self.parent
        {
            Some(parent)=>parent.have_child.set(false),
            None=>()
        }
    }
}





/*
pub struct Frame
{
    handle: FidT,
    parent: Box<Option<Frame>>
}



impl Frame
{
    //WARNING
    /**
    Create new frame.
    */
    pub fn new()->Self
    {
        Self::new_from_parent(None)
    }

    pub fn nest(self)->Self
    {
        Self::new_from_parent(Some(self))
    }

    fn new_from_parent(frame: Option<Frame>)->Self
    {
        Self
        {
            handle: open_foreign_frame(),
            parent: Box::new(frame)
        }
    }

    pub fn unnest(mut self)->Option<Frame>
    {
        self.parent.take()
    }

    pub fn create_term_refs(&self,count: usize)->TermRefs
    {
        TermRefs::new(&self,count)
    }
}
impl std::ops::Deref for Frame {
    type Target = FidT;
    fn deref(&self) -> &Self::Target {&self.handle}
}
impl Drop for Frame {
    fn drop(&mut self) {
        discard_foreign_frame(self.handle);
    }
}
*/

