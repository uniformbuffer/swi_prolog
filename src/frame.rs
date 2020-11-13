use crate::bindings::*;

#[cfg(log)]
use std::sync::atomic::{AtomicU32,Ordering};
#[cfg(log)]
static FRAME_COUNTER: AtomicU32 = AtomicU32::new(0);


use crate::term::TermRefs;
use std::cell::Cell;
use std::marker::PhantomData;
/**
Represent a prolog foreign frame. Frame allow to create term allocation and destroy all of them when deinitialized.
*/
pub struct Frame<'a>
{
    handle: FidT,
    parent: Option<Box<Frame<'a>>>,
    have_child: Cell<bool>,
    phantom: PhantomData<&'a usize>,
}
impl<'a> Frame<'a>
{
    pub fn new()->Self //parent: Option<&Frame<'a>>
    {
        let frame = Self
        {
            handle: open_foreign_frame(),
            parent: None,
            have_child: Cell::new(false),
            phantom: PhantomData
        };
        #[cfg(log)]
        log::trace!("Frame created: {:?}", FRAME_COUNTER.fetch_add(1,Ordering::Relaxed));

        frame
    }

    pub fn nest(&self)->Option<Frame<'a>>
    {
        if !self.have_child.get()
        {
            self.have_child.set(true);
            let frame = Self::new();
            Some(frame)
        }
        else {None}
    }

    pub fn create_term_refs(&self,size: usize)->Option<TermRefs<'a>>
    {
        if !self.have_child.get()
        {
            Some(TermRefs::new(size))
        }
        else {None}
    }
}

impl<'a> Drop for Frame<'a> {
    fn drop(&mut self) {
        #[cfg(log)]
        log::trace!("Frame destroyed: {:?}", FRAME_COUNTER.fetch_add(1,Ordering::Relaxed));

        discard_foreign_frame(self.handle);
        match &self.parent
        {
            Some(parent)=>parent.have_child.set(false),
            None=>()
        }
    }
}


