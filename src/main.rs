//Include macros to be able to use `insert_routes!`.
#[macro_use]
extern crate rustful;
extern crate unicase;

#[macro_use]
extern crate log;
extern crate env_logger;
//use rustful::file;

extern crate multipart;

extern crate bincode;
extern crate rustc_serialize;


use std::fmt::Write;

use rustful::StatusCode::{BadRequest, InternalServerError};

use multipart::server::MultipartData;

use std::path::Path;

use std::error::Error;

use rustful::{Server, Context,Handler, Response, TreeRouter};


extern crate time;

fn time_filename() -> String {
    let time = time::get_time();
    format!("/tmp/data-{}",time.nsec)
}

struct Api(Option<fn(Context,Response)>);


impl Handler for Api {
    fn handle_request(&self,context: Context, mut response: Response) {
        if let Some(action) = self.0 {
            action( context, response);
        }
    }
}

fn my_handler_get( context: Context,mut response: Response) {
    response.send(format!("<h1>Hello</h1>!"));
}

fn my_handler_post(mut context: Context, mut response: Response) {
//    println!("enter");
    let path = context.variables.get("path").unwrap_or("/".into()) ;
    
    println!("** adding file into : {}",path);

    match context.body.as_multipart() {
        Some(mut multipart) => {
            let mut result = String::new();
            
            //Iterate over the multipart entries and print info about them in `result`
             let _= multipart.foreach_entry(|entry| match entry.data {
                MultipartData::Text(text) => {
                    //Found data from a text field
                    let _ = writeln!(&mut result, "nothing done with {}: '{}'", entry.name, text);
                },
                MultipartData::File(mut file) => {
                    println!("found file");
                    let name = time_filename();
                    //let _ =  file.save();


                    match file.save_as(Path::new(&name)){
                        Ok(size) => if size==0 {response.set_status(InternalServerError);} ,
                        Err(x)     => {
                            println!("**\terror on writing file:\n\t{:?}",x);
                            response.set_status(InternalServerError)
                        },
                    }

                    //Found an uploaded file
                    
                    let mime = file.content_type();
                  //  writeln!(&mut result,"type: {}",mime);             
                                    
                    match file.filename(){
                        Some(file_name) => {
                            writeln!(&mut result, "{}: a file called '{}'", entry.name, file_name);
                        
                        } ,
                        None => {
                            response.set_status(InternalServerError);
                            println!("**\t error:unnamed file");
                            
                        }
                    }
                }
            });
            response.send(result);
        }, 
        None => {
            //We expected it to be a valid `multipart/form-data` request, but it was not
        response.set_status(BadRequest);
    }}
}
/*
 let mut router = insert_routes!{
        TreeRouter::new() => {
            Get: Api(Some(list_all)),
            Post: Api(Some(store)),
            Delete: Api(Some(clear)),
            Options: Api(None),
            ":id" => {
                Get: Api(Some(get_todo)),
                Patch: Api(Some(edit_todo)),
                Delete: Api(Some(delete_todo)),
                Options: Api(None)
            }
        }
    };


*/



fn main() {
    env_logger::init().unwrap();
    

    let my_router =insert_routes!{
            TreeRouter::new() => {
               "content" => {
                   "*path" => {
                       Get: Api(Some(my_handler_get)),
                       Post: Api(Some(my_handler_post)),
                   }
               },
            }
        };
        
    //Build and run the server.
    let server_result = Server {
        host: 8080.into(),
  //      content_type: content_type!(Text / Html; Charset = Utf8),
            handlers: my_router,
        ..Server::default()
    }.run();

    match server_result {
        Ok(_server) => {},
        Err(e) => error!("could not start server: {}", e.description())
    }
}

