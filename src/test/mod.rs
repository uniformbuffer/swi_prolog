extern crate tokio;

use crate::swi_prolog::SwiProlog;

#[test]
fn test_init()
{
    SwiProlog::new();
}

#[test]
fn test_get_default_module()
{
    let prolog = SwiProlog::new();
    let engine = prolog.get_engine();
    engine.get_module(&None);
}

#[test]
fn test_get_module()
{
    let prolog = SwiProlog::new();
    let engine = prolog.get_engine();
    engine.get_module(&Some("test module".to_string()));
}


#[test]
fn test_frame_i32()
{
    use crate::data::Data;
    use crate::term::Term;

    let prolog = SwiProlog::new();
    let engine = prolog.get_engine();
    let frame = engine.get_frame();

    let args = vec![Term::from(1i32),Term::from(2i32),Term::from(3i32)];
    let stack = frame.allocate_and_write(args).unwrap();

    assert!(Data::from(stack.get(0).unwrap()) == Data::from(1i32));
    assert!(Data::from(stack.get(1).unwrap()) == Data::from(2i32));
    assert!(Data::from(stack.get(2).unwrap()) == Data::from(3i32));
}


#[test]
fn test_frame_string()
{
    use crate::data::Data;
    use crate::term::Term;

    let prolog = SwiProlog::new();
    let engine = prolog.get_engine();
    let frame = engine.get_frame();

    let args = vec![Term::from("data1"),Term::from("data2"),Term::from("data3")];
    let stack = frame.allocate_and_write(args).unwrap();

    assert!(Data::from(stack.get(0).unwrap()) == Data::from("data1"));
    assert!(Data::from(stack.get(1).unwrap()) == Data::from("data2"));
    assert!(Data::from(stack.get(2).unwrap()) == Data::from("data3"));
}

#[test]
fn test_frame_string_i32()
{
    use crate::data::Data;
    use crate::term::Term;

    let prolog = SwiProlog::new();
    let engine = prolog.get_engine();
    let frame = engine.get_frame();

    let args = vec![Term::from("data1"),Term::from(2i32)];
    let stack = frame.allocate_and_write(args).unwrap();

    assert!(Data::from(stack.get(0).unwrap()) == Data::from("data1"));
    assert!(Data::from(stack.get(1).unwrap()) == Data::from(2i32));
}

/*
#[test]
fn test_query()
{
    use crate::data::Data;
    use crate::term::Term;

    let prolog = SwiProlog::new();

    let module_name = Some("test_query".to_string());

    let assert = prolog.run(&module_name,Term::assert(Term::from(("friend",vec![Term::from("person")]))));
    prolog.block_on(assert).unwrap().unwrap();

    let handle = prolog.query(&module_name,Term::from(("friend",vec![Term::from(())])));
    let result = prolog.block_on(handle).unwrap().unwrap();

    assert!(result.len() == 1);
    assert!(result.get(0).unwrap() == &vec![Data::from("person")]);
}
*/

#[test]
fn test_load_file_runtime()
{
    use crate::data::Data;
    use crate::term::Term;

    let prolog = SwiProlog::new();

    let module_name = Some("test_load_file_runtime".to_string());

    let assert = prolog.run(&module_name,Term::from(("load_files",vec![Term::from("./src/test/test_file")])));
    prolog.block_on(assert).unwrap().unwrap();

    let handle = prolog.query(&module_name,Term::from(("friend",vec![Term::from(())])));
    let result = prolog.block_on(handle).unwrap().unwrap();


    assert!(result.len() == 3);
    assert!(result.get(0).unwrap() == &vec![Data::from("person1")]);
    assert!(result.get(1).unwrap() == &vec![Data::from("person2")]);
    assert!(result.get(2).unwrap() == &vec![Data::from("person3")]);
}


/*
#[test]
fn test_assert()
{
    //let swipl = Swipl::new(vec!["./src/test/test_file.pl".to_string()]);
    let module = SwiProlog::new().get_module("./src/test/test_file.pl");
    let assert = SwiProlog::new().run(module,Term::assert(Term::from(("friend",vec![Term::from("matteo")]))));
    let query = SwiProlog::new().query(module,Term::from(("friend",vec![Term::from(())])));

    let (_assert_result,query_result) = SwiProlog::new().block_on(async{tokio::join!(assert,query)});
    assert!(query_result.unwrap().unwrap() == vec![
        vec![Data::from("person1")],
        vec![Data::from("person2")],
        vec![Data::from("person3")],
        vec![Data::from("person4")],
    ]);
}
*/
/*
#[test]
fn test_frame_i64()
{
    use crate::frame::Frame;

    //Needed to trigget the lazy allocation of SwiProlog::new(), so that initialize the system
    let prolog = SwiProlog::new();

    let args = vec![Term::from(1i64),Term::from(2i64),Term::from(3i64)];

    let frame = Frame::new();

    let mut stack = frame.create_term_refs(args.len());
    for i in 0..args.len() {
        let term = args.get(i).unwrap();
        term.write(&frame,stack.get_mut(i).unwrap());
    }

    assert!(Data::from(stack.get(0).unwrap()) == Data::from(1i64));
    assert!(Data::from(stack.get(1).unwrap()) == Data::from(2i64));
    assert!(Data::from(stack.get(2).unwrap()) == Data::from(3i64));
}

#[test]
fn test_exception()
{
    //let swipl = Swipl::new(vec!["./src/test/test_file.pl".to_string()]);
    //let module = SwiProlog::new().get_module("./src/test/test_file.pl");
    let module = SwiProlog::new().default_module();

    let handle = SwiProlog::new().query(module,Term::from(("a",vec![Term::from(1)])));
    let result = SwiProlog::new().block_on(handle).unwrap().unwrap();

}
*/
