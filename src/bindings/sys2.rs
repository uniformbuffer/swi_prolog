#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(improper_ctypes)]
//mod sys;
//pub use sys::*;

pub use crate::term::{TermRefs,TermAllocation};
pub use crate::functor::Functor;
pub use crate::data::Data;
pub use crate::module::Module;

use std::ffi::CString;
use std::os::raw::c_char;
use std::convert::TryFrom;

pub type TermT = usize;
pub type AtomT = usize;
pub type FunctorT = usize;
pub type ModuleT<'a> = &'a std::os::raw::c_void;
pub type PredicateT = *const std::os::raw::c_void;
pub type EngineT = *const std::os::raw::c_void;
pub type QidT = usize;
pub type FidT = usize;

/***** Initialization *****/
pub fn initialise<'a>(mut args: Vec<String>) -> Result<(),()>
{
    /*
    Special consideration is required for argv[0]. On Unix, this argument passes the part of the command line that is used to locate the executable.
    Prolog uses this to find the file holding the running executable. The Windows version uses this to find a module of the running executable.
    If the specified module cannot be found, it tries the module libpl.dll, containing the Prolog runtime kernel.
    */
    #[cfg(target_os = "linux")]
    args.insert(0,String::new());

    let cstring_vec: Vec<CString> = args.iter().map(|string|CString::new(string.as_str()).unwrap()).collect();

    let mut ptrs: Vec<*mut c_char> = cstring_vec.iter().map(|string|{
        unsafe{std::mem::transmute::<*const c_char,*mut c_char>(string.as_ptr())}
    }).collect();

    if unsafe{PL_initialise(ptrs.len() as i32,ptrs.as_mut_ptr())} == 1 {Ok(())}
    else {Err(())}
}
extern "C" {
    #[doc = "\t    EMBEDDING\t\t*"]
    pub fn PL_initialise(argc: ::std::os::raw::c_int,argv: *mut *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}

pub fn cleanup() -> bool
{
    if unsafe{PL_cleanup(0)} == 1 {true}
    else{false}
}
extern "C" {
    pub fn PL_cleanup(status: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}

pub fn halt() -> bool
{
    if unsafe{PL_halt(0)} == 1 {true}
    else {false}
}
extern "C" {
    pub fn PL_halt(status: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}

/*
pub fn toplevel() -> bool
{
    if unsafe{PL_toplevel()} == 1 {true}
    else {false}
}
*/

pub const PL_ENGINE_MAIN: EngineT = 0x1 as EngineT;
pub const PL_ENGINE_CURRENT: EngineT = 0x2 as EngineT;

pub const PL_ENGINE_SET: u32 = 0;
pub const PL_ENGINE_INVAL: u32 = 2;
pub const PL_ENGINE_INUSE: u32 = 3;

pub enum EngineSetResult
{
    Ok = 0,
    Invalid = 2,
    InUse = 3
}
impl TryFrom<i32> for EngineSetResult {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value
        {
            value if value == EngineSetResult::Ok as i32 =>Ok(EngineSetResult::Ok),
            value if value == EngineSetResult::Invalid as i32 =>Ok(EngineSetResult::Invalid),
            value if value == EngineSetResult::InUse as i32 =>Ok(EngineSetResult::InUse),
            _unknown => Err("Cannot convert value into EngineSetResult")
        }
    }
}

pub fn create_engine() -> EngineT
{
    unsafe{PL_create_engine(std::ptr::null_mut())}
}
extern "C" {
    pub fn PL_create_engine(attributes: *mut PL_thread_attr_t) -> EngineT;
}

pub fn get_engine() -> Option<EngineT>
{
    let mut current_engine = std::ptr::null();
    match EngineSetResult::try_from(unsafe{PL_set_engine(PL_ENGINE_CURRENT,&mut current_engine)}).unwrap()
    {
        EngineSetResult::Ok=>{}//println!("Engine getted successfully");
        _=>{panic!("Error on getting current engine")}
    }
    if current_engine != std::ptr::null() {Some(current_engine)}
    else {None}
}

pub fn set_engine(engine: EngineT)
{
    match EngineSetResult::try_from(unsafe{PL_set_engine(engine,&mut std::ptr::null())}).unwrap()
    {
        EngineSetResult::Ok=>{}//println!("Thread {:?} switched engine to {:#?}",std::thread::current().id(),engine);
        EngineSetResult::InUse=>panic!("Failed: Thread {:?} switched engine to {:#?}",std::thread::current().id(),engine),
        EngineSetResult::Invalid=>panic!("Invalid engine")
    }

}
extern "C" {
    pub fn PL_set_engine(engine: EngineT, old: *mut EngineT) -> ::std::os::raw::c_int;
}

pub fn destroy_engine(engine: EngineT) -> bool
{
    if unsafe{PL_destroy_engine(engine)} == 1 {true}
    else {false}
}
extern "C" {
    pub fn PL_destroy_engine(engine: EngineT) -> ::std::os::raw::c_int;
}




/***** Thread *****/
pub fn thread_self() -> Option<i32>
{
    let thread_id = unsafe{PL_thread_self()};
    if thread_id >= 0 {Some(thread_id)}
    else {None}
}
extern "C" {
    pub fn PL_thread_self() -> ::std::os::raw::c_int;
}
/*
pub const rc_cancel_PL_THREAD_CANCEL_FAILED: rc_cancel = 0;
pub const rc_cancel_PL_THREAD_CANCEL_JOINED: rc_cancel = 1;
pub const rc_cancel_PL_THREAD_CANCEL_MUST_JOIN: rc_cancel = 2;
*/
pub type RcCancel = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PL_thread_attr_t {
    pub stack_limit: usize,
    pub table_space: usize,
    pub alias: *mut ::std::os::raw::c_char,
    pub cancel: ::std::option::Option<unsafe extern "C" fn(id: ::std::os::raw::c_int) -> RcCancel>,
    pub flags: isize,
    pub max_queue_size: usize,
    pub reserved: [*mut ::std::os::raw::c_void; 3usize],
}

pub fn thread_attach_engine() -> ::std::os::raw::c_int
{
    unsafe{PL_thread_attach_engine(std::ptr::null_mut())}
}
extern "C" {
    pub fn PL_thread_attach_engine(attr: *mut PL_thread_attr_t) -> ::std::os::raw::c_int;
}

pub fn thread_destroy_engine() -> bool
{
    if unsafe{PL_thread_destroy_engine()} == 1 {true}
    else {false}
}
extern "C" {
    pub fn PL_thread_destroy_engine() -> ::std::os::raw::c_int;
}


//Allocation and deallocation of TermT
pub fn new_term_ref() -> TermT {unsafe{PL_new_term_ref()}}
extern "C" {
    pub fn PL_new_term_ref() -> TermT;
}

pub fn new_term_refs(count: usize) -> usize{unsafe{PL_new_term_refs(count as i32)}}
extern "C" {
    #[doc = "        TERM-REFERENCES\t*"]
    pub fn PL_new_term_refs(n: ::std::os::raw::c_int) -> TermT;
}

pub fn reset_term_refs(term: TermT) {unsafe{PL_reset_term_refs(term)};}
extern "C" {
    pub fn PL_reset_term_refs(r: TermT);
}

#[derive(Debug)]
pub enum TermType
{
    Bool = 17,
    Integer = 3,
    Long = 22,
    Double = 23,
    String = 6,
    Atom = 2,
}
impl TryFrom<i32> for TermType {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value
        {
            value if value == TermType::Bool as i32 =>Ok(TermType::Bool),
            value if value == TermType::Integer as i32 =>Ok(TermType::Integer),
            value if value == TermType::Long as i32 =>Ok(TermType::Long),
            value if value == TermType::Double as i32 =>Ok(TermType::Double),
            value if value == TermType::String as i32 =>Ok(TermType::String),
            value if value == TermType::Atom as i32 =>Ok(TermType::Atom),
            _unknown => Err("Cannot convert value into TermType")
        }
    }
}


pub fn term_type(term: TermT)->TermType
{
    TermType::try_from(unsafe{PL_term_type(term)}).unwrap()
}
extern "C" {
    pub fn PL_term_type(term: TermT) -> ::std::os::raw::c_int;
}


/***** Getters *****/
pub fn get_bool(term: TermT) -> Option<bool>
{
    let mut value = 0;
    if unsafe{PL_get_bool(term,&mut value)} == 1 {Some(if value == 1 {true} else {false})}
    else {None}
}
extern "C" {
    pub fn PL_get_bool(t: TermT, value: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}

pub fn get_i32(term: TermT) -> Option<i32>
{
    let mut value = 0;
    if unsafe{PL_get_integer(term,&mut value)} == 1 {Some(value)}
    else {None}
}
extern "C" {
    pub fn PL_get_integer(t: TermT, i: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}

pub fn get_i64(term: TermT) -> Option<i64>
{
    let mut value = 0;
    if unsafe{PL_get_int64(term,&mut value)} == 1 {Some(value)}
    else {None}
}
extern "C" {
    #[doc = "\t   WIDE INTEGERS\t*"]
    pub fn PL_get_int64(t: TermT, i: *mut i64) -> ::std::os::raw::c_int;
}

pub fn get_f64(term: TermT) -> Option<f64>
{
    let mut value = 0.0;
    if unsafe{PL_get_float(term,&mut value)} == 1 {Some(value)}
    else {None}
}
extern "C" {
    pub fn PL_get_float(t: TermT, f: *mut f64) -> ::std::os::raw::c_int;
}

pub fn get_string(term: TermT) -> Option<String>
{
    let mut tmp = std::ptr::null_mut();
    let mut len = 0;
    if unsafe{PL_get_string(term,&mut tmp,&mut len)} == 1 {Some(unsafe{CString::from_raw(tmp)}.into_string().unwrap())}
    else {None}
}
extern "C" {
    pub fn PL_get_string(
        t: TermT,
        s: *mut *mut ::std::os::raw::c_char,
        len: *mut usize,
    ) -> ::std::os::raw::c_int;
}

pub fn get_atom_chars(term: TermT) -> Option<String>
{
    let mut tmp = std::ptr::null_mut();
    if unsafe{PL_get_atom_chars(term,&mut tmp)} == 1 {Some(unsafe{CString::from_raw(tmp)}.into_string().unwrap())}
    else {None}
}
extern "C" {
    pub fn PL_get_atom_chars(
        t: TermT,
        a: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}

pub fn get_functor(term: TermT) -> Option<Functor>
{
    let mut functor = 0;
    if unsafe{PL_get_functor(term,&mut functor)} == 1 {Some(Functor::from_raw(functor))}
    else {None}
}
extern "C" {
    pub fn PL_get_functor(t: TermT, f: *mut FunctorT) -> ::std::os::raw::c_int;
}

/***** Setters *****/
pub fn put_bool(term: TermT, value: bool) {unsafe{PL_put_bool(term,value as i32)};}
extern "C" {
    pub fn PL_put_bool(t: TermT, val: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}

pub fn put_i32(term: TermT, value: i32){unsafe{PL_put_integer(term,value.into())};}
extern "C" {
    pub fn PL_put_integer(t: TermT, i: ::std::os::raw::c_long) -> ::std::os::raw::c_int;
}

pub fn put_i64(term: TermT, value: i64){unsafe{PL_put_int64(term,value)};}
extern "C" {
    pub fn PL_put_int64(t: TermT, i: i64) -> ::std::os::raw::c_int;
}

pub fn put_f64(term: TermT, value: f64){unsafe{PL_put_float(term,value)};}
extern "C" {
    pub fn PL_put_float(t: TermT, f: f64) -> ::std::os::raw::c_int;
}

pub fn put_string(term: TermT,value: String){unsafe{PL_put_atom_chars(term,CString::new(value.as_str()).unwrap().as_ptr())};}
extern "C" {
    pub fn PL_put_atom_chars(t: TermT,chars: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}

pub fn put_variable(term: TermT){unsafe{PL_put_variable(term)};}
extern "C" {
    pub fn PL_put_variable(t: TermT) -> ::std::os::raw::c_int;
}


/***** Functor *****/
pub fn new_functor<'a>(name: impl Into<&'a str>, ariety: usize)->FunctorT
{
    let name_atom = unsafe{PL_new_atom(CString::new(name.into()).unwrap().as_ptr())};
    return unsafe{PL_new_functor_sz(name_atom,ariety)};
}
extern "C" {
    pub fn PL_new_functor_sz(f: AtomT, a: usize) -> FunctorT;
}

pub fn functor_name(functor: FunctorT)->String
{
    let atom_name = unsafe{PL_functor_name(functor)};
    return atom_chars(atom_name);
}
extern "C" {
    pub fn PL_functor_name(f: FunctorT) -> AtomT;
}

pub fn functor_arity(functor: FunctorT)->usize
{
    unsafe{PL_functor_arity_sz(functor)}
}
extern "C" {
    pub fn PL_functor_arity_sz(f: FunctorT) -> usize;
}

pub fn cons_functor_v(functor: FunctorT, parameters: TermT, term: TermT)
{
    unsafe{PL_cons_functor_v(term,functor,parameters)};
}
extern "C" {
    pub fn PL_cons_functor_v(h: TermT, fd: FunctorT, a0: TermT) -> ::std::os::raw::c_int;
}

/***** Predicate *****/
pub fn pred(functor: FunctorT, module: ModuleT) -> PredicateT
{
    unsafe{PL_pred(functor,module)}
}
extern "C" {
    pub fn PL_pred(f: FunctorT, m: ModuleT) -> PredicateT;
}
/*
pub fn get_predicate<'a>(name: impl Into<&'a str>, arity: usize,module: impl Into<&'a str>)->PredicateT
{
    unsafe{PL_predicate(CString::new(name.into()).unwrap().as_ptr(),arity as i32,CString::new(module.into()).unwrap().as_ptr())}
}
*/
/***** Atom *****/
pub fn atom_chars(atom: AtomT)->String
{
    let ptr_const = unsafe{PL_atom_chars(atom)};
    let ptr_mut = unsafe{std::mem::transmute::<*const c_char,*mut c_char>(ptr_const)};
    return unsafe{CString::from_raw(ptr_mut)}.into_string().unwrap();
}
extern "C" {
    pub fn PL_atom_chars(a: AtomT) -> *const ::std::os::raw::c_char;
}

pub fn new_atom<'a>(value: impl Into<&'a str>) -> AtomT
{
    unsafe{PL_new_atom(CString::new(value.into()).unwrap().as_ptr())}
}
extern "C" {
    pub fn PL_new_atom(s: *const ::std::os::raw::c_char) -> AtomT;
}

/***** Query *****/
pub enum QueryFlags
{
    Normal = 2,
    NoDebug = 4,
    CatchException = 8,
    PassException = 16,
    AllowYield = 32,
    ExtStatus = 64,
}

#[cfg(all(debug_assertions,not(test)))]
const QUERY_FLAGS: QueryFlags = QueryFlags::Normal;

#[cfg(not(all(debug_assertions,not(test))))]
const QUERY_FLAGS: QueryFlags = QueryFlags::Normal;


pub fn call_predicate(module: ModuleT, predicate: PredicateT,terms: TermT)->Result<QueryResult,String>
{
    QueryResult::try_from(unsafe{PL_call_predicate(module, QUERY_FLAGS as i32, predicate, terms)})
/*
    match unsafe{PL_call_predicate(module, query_flags as i32, predicate, terms)}
    {
        PL_S_EXCEPTION=>Err(String::from("Error")),//TODO Missing query id to get exception with exception(qid)
        PL_S_FALSE_I32=>Ok(false),
        PL_S_TRUE_I32=>Ok(true),
        PL_S_LAST_I32=>Ok(true),
        unknown=>panic!("Unknown solution result {}",unknown)
    }
*/
}
extern "C" {
    pub fn PL_call_predicate(m: ModuleT,debug: ::std::os::raw::c_int,pred: PredicateT,t0: TermT) -> ::std::os::raw::c_int;
}

pub fn open_query(module: ModuleT, predicate: PredicateT,terms: TermT)->QidT
{
    unsafe{PL_open_query(module, QUERY_FLAGS as i32, predicate, terms)}
}
extern "C" {
    pub fn PL_open_query(m: ModuleT,flags: ::std::os::raw::c_int,pred: PredicateT,t0: TermT) -> QidT;
}


pub const PL_S_EXCEPTION: i32 = -1;
pub const PL_S_FALSE: i32 = 0;
pub const PL_S_TRUE: i32 = 1;
pub const PL_S_LAST: i32 = 2;

pub enum QueryResult
{
    True = 1,
    False = 2,
    Last = 3
}
impl TryFrom<i32> for QueryResult {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value
        {
            value if value == QueryResult::True as i32 =>Ok(QueryResult::True),
            value if value == QueryResult::False as i32 =>Ok(QueryResult::False),
            value if value == QueryResult::Last as i32 =>Ok(QueryResult::Last),
            _unknown => Err("Cannot convert value into QueryResult".to_string())
        }
    }
}

impl Into<bool> for QueryResult {
    fn into(self) -> bool {
        match self
        {
            Self::True=>true,
            Self::False=>false,
            Self::Last=>true
        }
    }
}

/*
const PL_S_FALSE_I32: i32 = PL_S_FALSE as i32;
const PL_S_TRUE_I32: i32 = PL_S_TRUE as i32;
const PL_S_LAST_I32: i32 = PL_S_LAST as i32;
*/
//pub fn next_solution(qid: QidT) -> Result<QueryResult,String> {QueryResult::try_from(unsafe{PL_next_solution(qid)})}
pub fn next_solution(qid: QidT) -> bool {unsafe{PL_next_solution(qid) != 0}}
extern "C" {
    pub fn PL_next_solution(qid: QidT) -> ::std::os::raw::c_int;
}

/*
pub fn cut_query(qid: QidT) -> Result<(),String>
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
pub fn close_query(qid: QidT) -> Result<QueryResult,String>
{
    QueryResult::try_from(unsafe{PL_close_query(qid)})
    /*
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
    */
}
extern "C" {
    pub fn PL_close_query(qid: QidT) -> ::std::os::raw::c_int;
}

pub fn exception(qid: QidT) -> String
{
    let allocation = unsafe{TermAllocation::from_raw(PL_exception(qid))};
    match Data::from(&allocation)
    {
        Data::String(s)=>s.clone(),
        _=>panic!("Unknown exception")
    }
}
extern "C" {
    pub fn PL_exception(qid: QidT) -> TermT;
}

/***** Frame *****/
pub fn open_foreign_frame() -> FidT {unsafe{PL_open_foreign_frame()}}
extern "C" {
    pub fn PL_open_foreign_frame() -> FidT;
}

pub fn close_foreign_frame(frame: FidT) {unsafe{PL_close_foreign_frame(frame)}}
extern "C" {
    pub fn PL_close_foreign_frame(cid: FidT);
}

pub fn discard_foreign_frame(frame: FidT) {unsafe{PL_discard_foreign_frame(frame)}}
extern "C" {
    pub fn PL_discard_foreign_frame(cid: FidT);
}

/***** Module *****/
pub fn context<'a>() -> ModuleT<'a> {unsafe{PL_context()}}
extern "C" {
    #[doc = "            MODULES            *"]
    pub fn PL_context<'a>() -> ModuleT<'a>;
}

pub fn new_module<'a,'b>(name: impl Into<&'b str>) -> ModuleT<'a> {unsafe{PL_new_module(new_atom(name))}}
extern "C" {
    pub fn PL_new_module<'a>(name: AtomT) -> ModuleT<'a>;
}

pub fn module_name(module: ModuleT)->String {atom_chars(unsafe{PL_module_name(module)})}
extern "C" {
    pub fn PL_module_name(module: ModuleT) -> AtomT;
}
