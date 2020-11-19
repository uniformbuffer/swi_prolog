use crate::bindings::*;

#[cfg(feature = "logger")]
use std::sync::atomic::{AtomicU32,Ordering};
#[cfg(feature = "logger")]
static FRAME_COUNTER: AtomicU32 = AtomicU32::new(0);

use crate::term::{Term,TermRefs};
use std::cell::Cell;
use std::marker::PhantomData;
/**
Represent a profeature = "logger" foreign frame. Frame allow to create term allocation and destroy all of them when deinitialized.
*/
pub struct Frame<'a>
{
    handle: FidT,
    parent: Option<Box<Frame<'a>>>,
    have_child: Cell<bool>,
    phantom: PhantomData<&'a usize>,
    #[cfg(feature = "logger")]
    id: u32,
    #[cfg(feature = "logger")]
    parent_id: u32
}
impl<'a> Frame<'a>
{
    pub fn new(
        #[cfg(feature = "logger")]
        parent_id: u32
    )->Self //parent: Option<&Frame<'a>>
    {
        let frame = Self
        {
            handle: open_foreign_frame(),
            parent: None,
            have_child: Cell::new(false),
            phantom: PhantomData,
            #[cfg(feature = "logger")]
            id: FRAME_COUNTER.fetch_add(1,Ordering::Relaxed),
            #[cfg(feature = "logger")]
            parent_id: parent_id
        };
        #[cfg(feature = "logger")]
        log::trace!("Frame {} created on engine ref {}", frame.id,parent_id);

        frame
    }

    pub fn nest(&self)->Option<Frame<'a>>
    {
        if !self.have_child.get()
        {
            self.have_child.set(true);
            let frame = Self::new(
                #[cfg(feature = "logger")]
                self.id
            );
            Some(frame)
        }
        else {None}
    }

    pub fn create_term_refs(&self,size: usize)->Option<TermRefs<'a>>
    {
        if !self.have_child.get() {Some(TermRefs::with_capacity(size))}
        else {None}
    }

    pub fn allocate_and_write(&self,terms: Vec<Term>)->Option<TermRefs<'a>>
    {
        if !self.have_child.get()
        {
            let mut allocation_count = 0;
            for term in &terms {allocation_count += Self::count_allocations(&term);}

            let mut term_refs = TermRefs::with_capacity(allocation_count);
            let mut offset = 0;
            for term in &terms {offset = Self::write_term(&mut term_refs,offset,&term);}
            Some(term_refs)
        }
        else {None}
    }

    fn write_term(term_refs: &mut TermRefs, offset: usize, term: &Term)->usize
    {
        match term
        {
            Term::Bool(value)=>{term_refs.get_mut(offset).unwrap().put_bool(*value);return offset+1;}
            Term::I32(value)=>{term_refs.get_mut(offset).unwrap().put_i32(*value);return offset+1;}
            Term::I64(value)=>{term_refs.get_mut(offset).unwrap().put_i64(*value);return offset+1;}
            Term::F64(value)=>{term_refs.get_mut(offset).unwrap().put_f64(*value);return offset+1;}
            Term::String(value)=>{term_refs.get_mut(offset).unwrap().put_string(value.clone());return offset+1;}
            Term::Variable=>{term_refs.get_mut(offset).unwrap().put_variable();return offset+1;}
            Term::Predicate(name,terms)=>
            {
                let mut local_offset = offset;
                for term in terms.iter() {local_offset = Self::write_term(term_refs,local_offset,term);}

                let functor = Functor::new(name.as_str(),terms.len());

                let functor_allocation = term_refs.get_mut(local_offset).unwrap();
                cons_functor_v(**functor_allocation,*functor,**term_refs.get_mut(offset).unwrap());
                return local_offset+1;
            }
        }
    }
    fn count_allocations(term: &Term)->usize
    {
        match &term
        {
            Term::Bool(_) | Term::I32(_) | Term::I64(_) | Term::F64(_) | Term::String(_) | Term::Variable =>1,
            Term::Predicate(_,terms)=>
            {
                let mut count = 0;
                for term in terms.iter() {count += Self::count_allocations(&term);}
                count
            }
        }
    }

}

impl<'a> Drop for Frame<'a> {
    fn drop(&mut self) {
        #[cfg(feature = "logger")]
        log::trace!("Frame {} destroyed on engine ref {}", self.id, self.parent_id);

        discard_foreign_frame(self.handle);
        match &self.parent
        {
            Some(parent)=>parent.have_child.set(false),
            None=>()
        }
    }
}


