use crate::bindings::*;

use crate::query::Query;
use crate::term::Term;
use crate::engine::{Engine,EngineRef};

use tokio::runtime::Builder;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

use lazy_static::lazy_static;

use std::sync::{Arc,Mutex,Condvar,atomic::{AtomicU32,Ordering}};


lazy_static! {
    pub static ref ENGINE: StaticSwiProlog = StaticSwiProlog::new();
}


pub struct StaticSwiProlog
{
    thread_pool: Runtime,
    cond_var: Condvar,
    engine_pool: Mutex<Vec<Engine>>,
    module_retrieve_lock: Mutex<()>
}
impl StaticSwiProlog
{
    pub(crate) fn new()->Self
    {
        #[cfg(feature = "logger")]
        crate::logger::init_logger();
        #[cfg(feature = "logger")]
        log::trace!("StaticSwiProlog created");

        initialise(Vec::new()).expect("Initialization Failed");

        let thread_count = Arc::new(AtomicU32::new(0));
        let thread_count_clone = thread_count.clone();
        let thread_pool = Builder::new_multi_thread()
            .on_thread_start(move ||{thread_count_clone.fetch_add(1,Ordering::Relaxed);})
            .build().unwrap();

        let mut engine_pool = Vec::new();
        for _i in 0..thread_count.load(std::sync::atomic::Ordering::Relaxed) {engine_pool.push(Engine::new())}
        Self
        {
            thread_pool: thread_pool,
            cond_var: Condvar::new(),
            engine_pool: Mutex::new(engine_pool),
            module_retrieve_lock: Mutex::new(())
        }
    }

    pub fn get_engine(&self)->EngineRef
    {
        let mut lock = self.engine_pool.lock().unwrap();
        match lock.pop()
        {
            Some(engine)=>{
                return EngineRef::new(engine);
            }
            None=>{
                loop
                {
                    lock = self.cond_var.wait(lock).unwrap();
                    match lock.pop()
                    {
                        Some(engine)=>{return EngineRef::new(engine);}
                        None=>()
                    }
                }
            }
        }
    }

    pub(crate) fn put_engine(&self,engine: Engine)
    {
        let mut engine_pool = self.engine_pool.lock().unwrap();
        engine_pool.push(engine);
        self.cond_var.notify_one();
    }

}
impl Drop for StaticSwiProlog {
    fn drop(&mut self) {
        #[cfg(log)]
        log::trace!("StaticSwiProlog destroyed");

        {let _engines: Vec<Engine> = self.engine_pool.lock().unwrap().drain(..).collect();}

        halt();
    }
}
impl std::ops::Deref for StaticSwiProlog {
    type Target = Runtime;
    fn deref(&self) -> &Self::Target {&self.thread_pool}
}

#[derive(Clone)]
pub struct SwiProlog(&'static StaticSwiProlog);
impl SwiProlog
{
    /**
    Initialize engine
    */
    pub fn new()->Self
    {
        Self(&ENGINE)
    }

    /**
    Make a query and return it's handle. The handle is async and results can be retrieved using SwiProlog::block_on function.
    */
    pub fn query(&self, module_name: Option<String>,term: Term)->JoinHandle<Result<Vec<Vec<Data>>,String>>
    {
        self.0.spawn(async move{
            let result = {
                let engine = ENGINE.get_engine();
                let frame = engine.get_frame();
                let module = engine.get_module(module_name);
                Query::query(&frame,module,term)
            };
            result
        })
    }
    /**
    Same as query function, but the query do not return data, only the result of the operation.
    */
    pub fn run(&self, module_name: Option<String>,term: Term)->JoinHandle<Result<bool,String>>
    {
        self.0.spawn(async move{
            let result = {
                let engine = ENGINE.get_engine();
                let frame = engine.get_frame();
                let module = engine.get_module(module_name);
                Query::run(&frame,module,term)
            };
            result
        })
    }
    /**
    Get direct control of an engine. Used only for test functions and engines should not be exposed to user.
    */
    pub(crate) fn get_engine(&self)->EngineRef {self.0.get_engine()}
}

impl std::ops::Deref for SwiProlog {
    type Target = Runtime;
    fn deref(&self) -> &Self::Target {&self.0}
}
