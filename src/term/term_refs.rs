use crate::bindings::*;
use crate::data::Data;
use std::marker::PhantomData;

#[cfg(feature = "logger")]
use std::sync::atomic::{AtomicU32,Ordering};
#[cfg(feature = "logger")]
static FRAME_COUNTER: AtomicU32 = AtomicU32::new(0);

pub struct TermRefs<'a>
{
    allocations: Vec<TermAllocation>,
    phantom: PhantomData<&'a usize>
}
impl<'a> TermRefs<'a>
{
    pub fn new()->Self {Self::with_capacity(0)}

    pub fn with_capacity(size: usize)->Self
    {
        let mut term_refs = Self
        {
            allocations: Vec::with_capacity(size),
            phantom: PhantomData
        };
        term_refs.extend(size);
        term_refs
    }

    pub fn get(&self,index: usize)->Option<&TermAllocation> {self.allocations.get(index)}
    pub fn get_mut(&mut self,index: usize)->Option<&mut TermAllocation> {self.allocations.get_mut(index)}

    pub fn len(&self)->usize {self.allocations.len()}

    pub fn extend(&mut self,count: usize)
    {
        if count == 0 {return;}

        let root = new_term_refs(count);
        match self.allocations.last()
        {
            Some(last)=>{if **last != root+1 {panic!("New allocations are not contiguous, so cannot be added to TermRefs");}}
            None=>()
        }
        for i in 0..count {self.allocations.push(TermAllocation(root+i));}
    }


    pub fn collect_variable_indexes(&self)->Vec<usize>
    {
        let mut indexes = Vec::new();
        for i in 0..self.allocations.len()
        {
            if self.allocations.get(i).unwrap().is_variable(){indexes.push(i);}
        }
        return indexes;
    }
}
impl<'a> std::ops::Deref for TermRefs<'a> {
    type Target = TermT;
    fn deref(&self) -> &Self::Target {&*self.allocations.get(0).unwrap()}
}


pub struct TermAllocation(TermT);
impl TermAllocation
{
    pub unsafe fn from_raw(term: TermT)->Self {Self(term)}
/*
    pub fn put(&mut self,term: impl Into<Term>)
    {
        match term.into()
        {
            Self::Bool(value)=>{self.put_bool(**allocation,*value);}
            Self::I32(value)=>{self.put_i32(**allocation,*value);}
            Self::I64(value)=>{self.put_i64(**allocation,*value);}
            Self::F64(value)=>{self.put_f64(**allocation,*value);}
            Self::String(value)=>{self.put_string(**allocation,value.clone());}
            Self::Variable=>{self.put_variable(**allocation);}
            Self::Predicate(name,terms)=>
            {
                let mut terms_allocation = frame.create_term_refs(terms.len()).unwrap();
                for i in 0..terms.len() {terms.get(i).unwrap().write(frame,terms_allocation.get_mut(i).unwrap());}

                let functor = Functor::new(name.as_str(),terms.len());
                cons_functor_v(*functor,*terms_allocation,**allocation)
            }
        }
    }
    */
    pub fn put_bool(&mut self,value: bool){put_bool(**self,value);}
    pub fn put_i32(&mut self,value: i32){put_i32(**self,value);}
    pub fn put_i64(&mut self,value: i64){put_i64(**self,value);}
    pub fn put_f64(&mut self,value: f64){put_f64(**self,value);}
    pub fn put_string(&mut self,value: String){put_string(**self,value);}
    pub fn put_variable(&mut self){put_variable(**self);}

    pub fn is_variable(&self)->bool{is_variable(**self)}
}
impl std::ops::Deref for TermAllocation {
    type Target = usize;
    fn deref(&self) -> &Self::Target {&self.0}
}


impl From<&TermAllocation> for Data
{
    fn from(term: &TermAllocation) -> Self {
        match term_type(**term)
        {
            TermType::Bool=>Self::Bool(get_bool(**term).unwrap()),
            TermType::Integer=>Self::I32(get_i32(**term).unwrap()),
            TermType::Long=>Self::I64(get_i64(**term).unwrap()),
            TermType::Double=>Self::F64(get_f64(**term).unwrap()),
            TermType::String=>Self::String(get_string(**term).unwrap()),
            TermType::Atom=>Self::String(get_atom_chars(**term).unwrap()),
            //unknown=>panic!("Failed to get Data from &TermAllocation: unknown type {:#?}",unknown)
        }
    }
}
