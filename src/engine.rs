use crate::bindings::*;

use crate::frame::Frame;

use crate::swi_prolog::ENGINE;

#[cfg(feature = "logger")]
use std::sync::atomic::{AtomicU32,Ordering};
#[cfg(feature = "logger")]
static ENGINE_COUNTER: AtomicU32 = AtomicU32::new(0);
#[cfg(feature = "logger")]
static ENGINE_REFS_COUNTER: AtomicU32 = AtomicU32::new(0);


pub struct Engine
{
    handle: EngineT,
    #[cfg(feature = "logger")]
    id: u32
}
impl Engine
{
    pub fn new()->Self
    {
        let engine = Self
        {
            handle: create_engine(),
            #[cfg(feature = "logger")]
            id: ENGINE_COUNTER.fetch_add(1,Ordering::Relaxed)
        };


        #[cfg(feature = "logger")]
        log::trace!("Engine {} created", engine.id);

        engine
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        #[cfg(feature = "logger")]
        log::trace!("Engine {} destroyed", self.id);

        destroy_engine(self.handle);
    }
}
impl std::ops::Deref for Engine {
    type Target = EngineT;
    fn deref(&self) -> &Self::Target {&self.handle}
}

unsafe impl Send for Engine {}
unsafe impl Sync for Engine {}



pub struct EngineRef<'a>(Option<Engine>,Option<Frame<'a>>,
#[cfg(feature = "logger")]
u32
);
impl<'a> EngineRef<'a>
{
    pub fn new(engine: Engine)->Self {
        set_engine(*engine);

        #[cfg(feature = "logger")]
        let id = ENGINE_COUNTER.fetch_add(1,Ordering::Relaxed);

        #[cfg(feature = "logger")]
        log::trace!("Engine ref {} created on engine {}", id, engine.id);

        let engine_ref = Self(
            Some(engine),
            Some(Frame::new(
                #[cfg(feature = "logger")]
                id
            )),
            #[cfg(feature = "logger")]
            id
        );



        engine_ref
    }

    pub fn get_module(&self,name: &Option<String>)->Module<'a>{
        match name
        {
            Some(name)=>Module::new(new_module(name.as_str())),
            None=>Module::new(context())
        }
    }
    pub fn get_frame(&self)->&Frame {self.1.as_ref().unwrap()}
}
impl<'a> Drop for EngineRef<'a> {
    fn drop(&mut self)
    {
        {self.1.take().unwrap();}

        #[cfg(feature = "logger")]
        log::trace!("Engine ref {} destroyed on engine {}",self.2,self.0.as_ref().unwrap().id);

        set_engine(std::ptr::null());
        ENGINE.put_engine(self.0.take().unwrap());
    }
}
impl<'a> std::ops::Deref for EngineRef<'a> {
    type Target = Engine;
    fn deref(&self) -> &Self::Target {&self.0.as_ref().unwrap()}
}


