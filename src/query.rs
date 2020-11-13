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
        let (predicate,args) = match &term
        {
            Term::Predicate(name,args)=>(Predicate::new(&module,name.as_str(),args.len()),args),
            _=>{return Err(String::from("Root term must be Term::Predicate"))}
        };

        // Store terms in term_refs and store indices of Term::Variable
        let mut variable_indexes = Vec::new();
        let mut stack = frame.create_term_refs(args.len()).expect("Allocation requested on frame that is no more a leaf, allocations must be requested on the leaf Frame");
        for i in 0..args.len() {
            let term = args.get(i).unwrap();
            term.write(frame,stack.get_mut(i).unwrap());
            match term
            {
                Term::Variable=>variable_indexes.push(i),
                _=>()
            }
        }

        //Open query and get the results from stored Term::Variable indices
        let query_id = open_query(*module,*predicate, *stack);
        let mut results = Vec::new();
        while next_solution(query_id)
        {
            let mut result = Vec::new();
            for i in &variable_indexes {result.push(Data::from(stack.get(*i).unwrap()));}
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
        let (predicate,args) = match &term
        {
            Term::Predicate(name,args)=>(Predicate::new(&module,name.as_str(),args.len()),args),
            _=>{return Err(String::from("Root term must be Term::Predicate"))}
        };

        // Store terms in term_refs
        let mut stack = frame.create_term_refs(args.len()).expect("Allocation requested on frame that is no more a leaf, allocations must be requested on the leaf Frame");
        for i in 0..args.len() {args.get(i).unwrap().write(frame,stack.get_mut(i).unwrap());}

        //Call predicate and return the result
        match call_predicate(*module,*predicate, *stack)
        {
            Ok(query_result)=>Ok(query_result.into()),
            Err(error)=>Err(error)
        }
    }

}


