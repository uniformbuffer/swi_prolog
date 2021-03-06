use crate::bindings::*;

use crate::data::Data;
use crate::predicate::Predicate;
use crate::term::Term;
use crate::frame::Frame;

pub struct Query;
impl Query
{

    pub fn query(frame: &Frame,module: Module,term: Term)->Result<Vec<Vec<Data>>,String>
    {
        //Getting predicate from root term
        let predicate = match &term
        {
            Term::Predicate(name,args)=>Predicate::new(&module,name.as_str(),args.len()),
            _=>{return Err(String::from("Root term must be Term::Predicate"))}
        };

        let term_refs = match frame.allocate_and_write(vec![term])
        {
            Some(term_refs)=>term_refs,
            None=>return Err(String::from("Failed to allocate terms"))
        };
        let variable_indexes = term_refs.collect_variable_indexes();

        //Open query and get the results from stored Term::Variable indices
        let query_id = open_query(*module,*predicate, *term_refs);
        let mut results = Vec::new();
        while next_solution(query_id)
        {
            let mut result = Vec::new();
            for i in &variable_indexes {result.push(Data::from(term_refs.get(*i).unwrap()));}
            results.push(result);
        }

        //Close query and return the result
        match close_query(query_id)
        {
            Ok(_)=>Ok(results),
            Err(exception)=>Err(exception)
        }
    }
    pub fn run(frame: &Frame,module: Module,term: Term)->Result<bool,String>
    {
        //Getting predicate from root term
        let predicate = match &term
        {
            Term::Predicate(name,args)=>Predicate::new(&module,name.as_str(),args.len()),
            _=>{return Err(String::from("Root term must be Term::Predicate"))}
        };

        // Store terms in term_refs
        let term_refs = match frame.allocate_and_write(vec![term])
        {
            Some(term_refs)=>term_refs,
            None=>return Err(String::from("Failed to allocate terms"))
        };

        //Call predicate and return the result
        match call_predicate(*module,*predicate, *term_refs)
        {
            Ok(query_result)=>Ok(query_result.into()),
            Err(error)=>Err(error)
        }
    }

}


