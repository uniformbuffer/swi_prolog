mod term_refs;
pub use term_refs::{TermRefs,TermAllocation};

use crate::bindings::*;
use crate::functor::Functor;
use crate::frame::Frame;
use std::rc::Rc;

#[derive(Clone,Debug)]
pub enum Term
{
    Bool(bool),
    I32(i32),
    I64(i64),
    F64(f64),
    String(String),
    Variable,
    Predicate(String,Box<Vec<Term>>)
}

impl Term
{
    pub fn assert(term: Term)->Term {Term::from(("assert",vec![term]))}
    pub fn retract(term: Term)->Term {Term::from(("retract",vec![term]))}
    pub fn head_body(term1: Term,term2: Term)->Term {Term::from((":-",vec![term1,term2]))}

    pub fn write(&self,frame: &Rc<Frame>, allocation: &mut TermAllocation)
    {
        match self
        {
            Self::Bool(value)=>{put_bool(**allocation,*value);}
            Self::I32(value)=>{put_i32(**allocation,*value);}
            Self::I64(value)=>{put_i64(**allocation,*value);}
            Self::F64(value)=>{put_f64(**allocation,*value);}
            Self::String(value)=>{put_string(**allocation,value.clone());}
            Self::Variable=>{put_variable(**allocation);}
            Self::Predicate(name,terms)=>
            {
                let mut terms_allocation = frame.create_term_refs(terms.len()).unwrap();
                for i in 0..terms.len() {terms.get(i).unwrap().write(frame,terms_allocation.get_mut(i).unwrap());}

                let functor = Functor::new(name.as_str(),terms.len());
                cons_functor_v(*functor,*terms_allocation,**allocation)
            }
            //unknown=>panic!("Unsupported Data: {:#?}",unknown)
        }
    }
}

impl From<String> for Term {fn from(value: String) -> Self {Self::String(value)}}
impl From<&str> for Term {fn from(value: &str) -> Self {Self::String(value.to_string())}}
impl From<bool> for Term {fn from(value: bool) -> Self {Self::Bool(value)}}
impl From<i32> for Term {fn from(value: i32) -> Self {Self::I32(value)}}
impl From<i64> for Term {fn from(value: i64) -> Self {Self::I64(value)}}
impl From<f64> for Term {fn from(value: f64) -> Self {Self::F64(value)}}
impl From<(String,Vec<Term>)> for Term {fn from(value: (String,Vec<Term>)) -> Self {Self::Predicate(value.0,Box::new(value.1))}}
impl From<(&str,Vec<Term>)> for Term {fn from(value: (&str,Vec<Term>)) -> Self {Self::Predicate(value.0.to_string(),Box::new(value.1))}}
impl From<()> for Term {fn from(_value: ()) -> Self {Self::Variable}}
