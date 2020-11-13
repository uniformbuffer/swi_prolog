
mod sys2;
pub use sys2::*;

/*
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(improper_ctypes)]
mod sys;
pub use sys::*;

pub use crate::term::{TermRefs,TermAllocation};
pub use crate::functor::Functor;
pub use crate::data::Data;
pub use crate::module::Module;

use std::ffi::CString;
use std::os::raw::c_char;

/***** Initialization *****/
pub fn initialise<'a>(mut args: Vec<String>) -> Result<(),()>
{
    /*
    Special consideration is required for argv[0]. On Unix, this argument passes the part of the command line that is used to locate the executable.
    Prolog uses this to find the file holding the running executable. The Windows version uses this to find a module of the running executable.
    If the specified module cannot be found, it tries the module libpl.dll, containing the Prolog runtime kernel.
    */
    #[cfg(target_os = "linux")]
    {args.insert(0,String::new());}

    args.push(String::from("--no-debug"));

    let cstring_vec: Vec<CString> = args.iter().map(|string|CString::new(string.as_str()).unwrap()).collect();

    let mut ptrs: Vec<*mut c_char> = cstring_vec.iter().map(|string|{
        unsafe{std::mem::transmute::<*const c_char,*mut c_char>(string.as_ptr())}
    }).collect();

    if unsafe{PL_initialise(ptrs.len() as i32,ptrs.as_mut_ptr())} == 1 {Ok(())}
    else {Err(())}
}

pub fn cleanup() -> bool
{
    if unsafe{PL_cleanup(0)} == 1 {true}
    else{false}
}

pub fn halt() -> bool
{
    if unsafe{PL_halt(0)} == 1 {true}
    else {false}
}

pub fn toplevel() -> bool
{
    if unsafe{PL_toplevel()} == 1 {true}
    else {false}
}

pub fn create_engine() -> PL_engine_t
{
    unsafe{PL_create_engine(std::ptr::null_mut())}
}
pub fn destroy_engine(engine: PL_engine_t) -> bool
{
    if unsafe{PL_destroy_engine(engine)} == 1 {true}
    else {false}
}

/***** Thread *****/
pub fn thread_self() -> Option<i32>
{
    let thread_id = unsafe{PL_thread_self()};
    if thread_id >= 0 {Some(thread_id)}
    else {None}
}

pub fn thread_attach_engine(attributes: Option<PL_thread_attr_t>) -> ::std::os::raw::c_int
{
    match attributes
    {
        Some(mut attributes)=>unsafe{PL_thread_attach_engine(&mut attributes)},
        None=>unsafe{PL_thread_attach_engine(std::ptr::null_mut())}
    }
}
pub fn thread_destroy_engine() -> bool
{
    if unsafe{PL_thread_destroy_engine()} == 1 {true}
    else {false}
}


//Allocation and deallocation of term_t
pub fn new_term_ref() -> term_t {unsafe{PL_new_term_ref()}}
pub fn new_term_refs(count: usize) -> usize{unsafe{PL_new_term_refs(count as i32)}}
pub fn reset_term_refs(term: term_t) {unsafe{PL_reset_term_refs(term)};}




/***** Getters *****/
pub fn get_bool(term: term_t) -> Option<bool>
{
    let mut value = 0;
    if unsafe{PL_get_bool(term,&mut value)} == 1 {Some(if value == 1 {true} else {false})}
    else {None}
}
pub fn get_i32(term: term_t) -> Option<i32>
{
    let mut value = 0;
    if unsafe{PL_get_integer(term,&mut value)} == 1 {Some(value)}
    else {None}
}
pub fn get_i64(term: term_t) -> Option<i64>
{
    let mut value = 0;
    if unsafe{PL_get_int64(term,&mut value)} == 1 {Some(value)}
    else {None}
}
pub fn get_f64(term: term_t) -> Option<f64>
{
    let mut value = 0.0;
    if unsafe{PL_get_float(term,&mut value)} == 1 {Some(value)}
    else {None}
}
pub fn get_string(term: term_t) -> Option<String>
{
    let mut tmp = std::ptr::null_mut();
    let mut len = 0;
    if unsafe{PL_get_string(term,&mut tmp,&mut len)} == 1 {Some(unsafe{CString::from_raw(tmp)}.into_string().unwrap())}
    else {None}
}
pub fn get_atom_chars(term: term_t) -> Option<String>
{
    let mut tmp = std::ptr::null_mut();
    if unsafe{PL_get_atom_chars(term,&mut tmp)} == 1 {Some(unsafe{CString::from_raw(tmp)}.into_string().unwrap())}
    else {None}
}
pub fn get_functor(term: term_t) -> Option<Functor>
{
    let mut functor = 0;
    if unsafe{PL_get_functor(term,&mut functor)} == 1 {Some(Functor::from_raw(functor))}
    else {None}
}

/***** Setters *****/
pub fn put_bool(term: term_t, value: bool) {unsafe{PL_put_bool(term,value as i32)};}
pub fn put_i32(term: term_t, value: i32){unsafe{PL_put_integer(term,value.into())};}
pub fn put_i64(term: term_t, value: i64){unsafe{PL_put_int64(term,value)};}
pub fn put_f64(term: term_t, value: f64){unsafe{PL_put_float(term,value)};}
pub fn put_string(term: term_t,value: String){unsafe{PL_put_atom_chars(term,CString::new(value.as_str()).unwrap().as_ptr())};}
pub fn put_variable(term: term_t){unsafe{PL_put_variable(term)};}


/***** Functor *****/
pub fn new_functor<'a>(name: impl Into<&'a str>, ariety: usize)->functor_t
{
    let name_atom = unsafe{PL_new_atom(CString::new(name.into()).unwrap().as_ptr())};
    return unsafe{PL_new_functor(name_atom,ariety as i32)};
}
pub fn functor_name(functor: functor_t)->String
{
    let atom_name = unsafe{PL_functor_name(functor)};
    return atom_chars(atom_name);
}
pub fn functor_arity(functor: functor_t)->usize
{
    return unsafe{PL_functor_arity(functor)} as usize;
}

pub fn cons_functor_v(functor: functor_t, parameters: term_t, term: term_t)
{
    unsafe{PL_cons_functor_v(term,functor,parameters)};
}

/***** Predicate *****/
pub fn pred(functor: &Functor, module: &Module) -> predicate_t
{
    unsafe{PL_pred(**functor,**module)}
}
pub fn get_predicate<'a>(name: impl Into<&'a str>, arity: usize,module: impl Into<&'a str>)->predicate_t
{
    unsafe{PL_predicate(CString::new(name.into()).unwrap().as_ptr(),arity as i32,CString::new(module.into()).unwrap().as_ptr())}
}

/***** Atom *****/
pub fn atom_chars(atom: atom_t)->String
{
    let ptr_const = unsafe{PL_atom_chars(atom)};
    let ptr_mut = unsafe{std::mem::transmute::<*const c_char,*mut c_char>(ptr_const)};
    return unsafe{CString::from_raw(ptr_mut)}.into_string().unwrap();
}
pub fn new_atom<'a>(value: impl Into<&'a str>) -> atom_t
{
    unsafe{PL_new_atom(CString::new(value.into()).unwrap().as_ptr())}
}

/***** Query *****/



pub fn call_predicate(predicate: predicate_t,terms: term_t)->Result<bool,String>
{
    match unsafe{PL_call_predicate(std::ptr::null_mut(), PL_Q_NORMAL as i32, predicate, terms)}
    {
        PL_S_EXCEPTION=>Err(String::from("Error")),//TODO Missing query id to get exception with exception(qid)
        PL_S_FALSE_I32=>Ok(false),
        PL_S_TRUE_I32=>Ok(true),
        PL_S_LAST_I32=>Ok(true),
        unknown=>panic!("Unknown solution result {}",unknown)
    }
}

pub fn open_query(predicate: predicate_t,terms: term_t)->qid_t
{
    unsafe{PL_open_query(std::ptr::null_mut(), PL_Q_NORMAL as i32, predicate, terms)}
}

const PL_S_FALSE_I32: i32 = PL_S_FALSE as i32;
const PL_S_TRUE_I32: i32 = PL_S_TRUE as i32;
const PL_S_LAST_I32: i32 = PL_S_LAST as i32;
pub fn next_solution(qid: qid_t) -> Result<bool,String>
{
    unsafe{
        match PL_next_solution(qid)
        {
            PL_S_EXCEPTION=>Err(exception(qid)),
            PL_S_FALSE_I32=>Ok(false),
            PL_S_TRUE_I32=>Ok(true),
            PL_S_LAST_I32=>Ok(true),
            unknown=>panic!("Unknown solution result {}",unknown)
        }
    }
}
/*
pub fn cut_query(qid: qid_t) -> Result<(),String>
{
    unsafe{
        if PL_cut_query(qid) == 1 {Ok(())}
        else {
            //Exception occur
            Err(exception(qid))
        }
    }
}
*/
pub fn close_query(qid: qid_t) -> Result<bool,String>
{
    unsafe{
        match PL_close_query(qid)
        {
            PL_S_EXCEPTION=>Err(exception(qid)),
            PL_S_FALSE_I32=>Ok(false),
            PL_S_TRUE_I32=>Ok(true),
            PL_S_LAST_I32=>Ok(true),
            unknown=>panic!("Unknown solution result {}",unknown)
        }
    }
}

pub fn exception(qid: qid_t) -> String
{
    let allocation = unsafe{TermAllocation::from_raw(PL_exception(qid))};
    match Data::from(&allocation)
    {
        Data::String(s)=>s.clone(),
        _=>panic!("Unknown exception")
    }
}

/***** Frame *****/
pub fn open_foreign_frame() -> PL_fid_t {unsafe{PL_open_foreign_frame()}}
pub fn close_foreign_frame(frame: PL_fid_t) {unsafe{PL_close_foreign_frame(frame)}}
pub fn discard_foreign_frame(frame: PL_fid_t) {unsafe{PL_discard_foreign_frame(frame)}}

/***** Module *****/
pub fn context() -> module_t {unsafe{PL_context()}}
pub fn new_module<'a>(name: impl Into<&'a str>) -> module_t {unsafe{PL_new_module(new_atom(name))}}
*/
