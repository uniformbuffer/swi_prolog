use crate::bindings::*;
use crate::data::Data;
use std::marker::PhantomData;

pub struct TermRefs<'a>
{
    allocations: Vec<TermAllocation>,
    phantom: PhantomData<&'a usize>
}
impl<'a> TermRefs<'a>
{
    pub fn new(size: usize)->Self
    {
        let root = new_term_refs(size);
        let mut allocations = Vec::new();
        for i in 0..size {allocations.push(TermAllocation(root+i));}
        Self
        {
            allocations: allocations,
            phantom: PhantomData
        }
    }

    pub fn get(&self,index: usize)->Option<&TermAllocation> {self.allocations.get(index)}
    pub fn get_mut(&mut self,index: usize)->Option<&mut TermAllocation> {self.allocations.get_mut(index)}

    pub fn len(&self)->usize {self.allocations.len()}
}
/*
impl<'a> Drop for TermRefs<'a> {
    fn drop(&mut self)
    {
        println!("TermRefs destroyed: {}",**self);
    }
}
*/
impl<'a> std::ops::Deref for TermRefs<'a> {
    type Target = TermT;
    fn deref(&self) -> &Self::Target {&*self.allocations.get(0).unwrap()}
}


pub struct TermAllocation(TermT);
impl TermAllocation
{
    pub unsafe fn from_raw(term: TermT)->Self {Self(term)}
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
