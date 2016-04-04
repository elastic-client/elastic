#![feature(rustc_private, quote)]

extern crate syntax;
extern crate aster;
extern crate elastic_codegen;
extern crate walkdir;

use std::error::Error;
use std::fs;
use std::fs::{ File, OpenOptions };
use std::io::Write;
use std::collections::HashMap;
use syntax::ast::*;
use syntax::ext::base::ExtCtxt;
use syntax::parse::token;
use syntax::codemap::DUMMY_SP;
use syntax::parse::token::intern;
use elastic_codegen::api::ast::*;
use elastic_codegen::api::parse::*;
use elastic_codegen::api::gen::rust::*;
use elastic_codegen::gen::rust::*;
use elastic_codegen::api::gen::parse::{ parse_path_parts, parse_path_params };
use elastic_codegen::emit::*;
use elastic_codegen::emit::rust::*;
use walkdir::WalkDir;

use std::env;

macro_rules! get_ctxt {
	($cx:ident, $ps:ident, $fgc:ident) => {
		$cx = ExtCtxt::new(
			&$ps, vec![],
			syntax::ext::expand::ExpansionConfig::default("qquote".to_string()),
			&mut $fgc
		);
		$cx.bt_push(syntax::codemap::ExpnInfo {
			call_site: DUMMY_SP,
			callee: syntax::codemap::NameAndSpan {
				format: syntax::codemap::MacroBang(intern("")),
				allow_internal_unstable: false,
				span: None,
			}
		});
	};
}

fn main() {
	let mut args = env::args();
	let _ = args.next().unwrap();
	let indir = args.next().unwrap();
	let outdir = args.next().unwrap();
	
	println!("spec: {}", indir);
	println!("output: {}", outdir);
	
	gen_from_source(&indir, &outdir).unwrap();
}

fn gen_from_source(source_dir: &str, dest_dir: &str) -> Result<(), String> {
	//Clear out the contents of the dest_dir
	println!("clearing destination dir...");
	let _ = fs::remove_dir_all(dest_dir).map_err(|e| e.description().to_string());
	let _ = fs::create_dir_all(dest_dir).map_err(|e| e.description().to_string());
	
	//Create an emitter and Execution Context
	let ps = syntax::parse::ParseSess::new();
	let mut fgc = vec![];
	let mut cx;
	get_ctxt!(cx, ps, fgc);
	
	let ps = syntax::parse::ParseSess::new();
	let mut fgc = vec![];
	let mut _cx;
	get_ctxt!(_cx, ps, fgc);
	let emitter = RustEmitter::new(_cx);
	
	//Get the spec source
	println!("parsing source spec files...");
	let parsed = try!(from_dir(source_dir).map_err(|e| e.description().to_string()));
	
	for endpoint in parsed {
		//1. Get the path for the generated source
		println!("building path for {}...", endpoint.get_name());
		let mut path = try!(endpoint.get_mod_path().map_err(|_| format!("Error parsing path for {}", endpoint.get_name())));    
		let (file, file_is_mod) = match path.len() {
			0 => ("mod".to_string(), true),
			1 => ("mod".to_string(), true),
			_ => (try!(path.pop().ok_or(format!("Error parsing path filename for {}", endpoint.get_name()))), false)
		};
		
		let dir_path = format!("{}/{}", dest_dir, path.join("/"));
		let file_path = format!("{}/{}.rs", dir_path, file);
		
		//Ensure the path exists
		try!(fs::create_dir_all(&dir_path).map_err(|e| e.description().to_string()));
		
		//2. Open the source file
		let (mut src_file, is_new) = match OpenOptions::new().write(true).append(true).open(&file_path) {
			Ok(f) => {
				println!("Opened file...");
				(f, false)
			},
			Err(_) => {
				println!("Creating file...");
				(try!(File::create(&file_path).map_err(|e| e.description().to_string())), true)
			}
		};
		
		//4. Emit file header for new files
		if is_new {
			println!("emitting header for {}...", endpoint.get_name());
			try!(emitter.emit_str(&format!("//! {}\n\n", endpoint.documentation), &mut src_file).map_err(|e| e.description().to_string()));

			try!(emitter.emit_str(&"//Autogenerated\n\n", &mut src_file).map_err(|e| e.description().to_string()));

			try!(emitter.emit(&quote_stmt!(&mut cx, use hyper::client::Client;), &mut src_file).map_err(|e| e.description().to_string()));
			try!(emitter.emit_str(&"\n", &mut src_file).map_err(|e| e.description().to_string()));

			try!(emitter.emit(&quote_stmt!(&mut cx, use hyper::header::{ Headers, ContentType };), &mut src_file).map_err(|e| e.description().to_string()));
			try!(emitter.emit_str(&"\n", &mut src_file).map_err(|e| e.description().to_string()));
			
			try!(emitter.emit(&quote_stmt!(&mut cx, use hyper::client::response::Response;), &mut src_file).map_err(|e| e.description().to_string()));
			try!(emitter.emit_str(&"\n", &mut src_file).map_err(|e| e.description().to_string()));
			
			try!(emitter.emit(&quote_stmt!(&mut cx, use hyper::error::Result;), &mut src_file).map_err(|e| e.description().to_string()));
			try!(emitter.emit_str(&"\n\n", &mut src_file).map_err(|e| e.description().to_string()));

			try!(emitter.emit(&quote_stmt!(&mut cx, use ::RequestParams;), &mut src_file).map_err(|e| e.description().to_string()));
			try!(emitter.emit_str(&"\n\n", &mut src_file).map_err(|e| e.description().to_string()));
			
			try!(src_file.sync_all().map_err(|e| e.description().to_string()));
		}
		
		//5. Generate and emit source functions
		let fun_sigs = try!(endpoint.get_fns().map_err(|e| e.description().to_string()));
		let mut fun_sigs_distinct = HashMap::with_capacity(fun_sigs.len());
		for fun in fun_sigs {
			fun_sigs_distinct.insert(fun.name.clone(), fun);
		}

		for (_, fun) in fun_sigs_distinct {
			println!("emitting fn {}", &fun.name);
			
			//The base url argument
			let _client = "client";
			let client = token::str_to_ident(_client);

			let req = token::str_to_ident("req");
			let base = token::str_to_ident("base");
			let body = token::str_to_ident("body");
			let qry = token::str_to_ident("url_qry");
			
			let lifetime = lifetime("'a");
			
			let mut params: Vec<Ident> = parse_path_params(&fun.path)
				.unwrap().iter()
				.map(|p| token::str_to_ident(match p.as_str() {
					"type" => "_type",
					s => s
				}))
				.collect();

			//Add the query string param so it's included when building full url
			params.push(qry);

			let parts = parse_path_parts(&fun.path).unwrap();
			
			//Get the push statements
			let (url_ident, url_stmts) = url_push_decl(base, parts.iter().map(|p| p.as_str()), params.to_vec());

			//Remove the query string param so it's not included in fn signature
			let _ = params.pop();

			//Function signature from params
			let mut rs_fun = build_fn(&fun.name, vec![
				build_arg(_client, build_ty_ptr("Client", Mutability::Mutable, Some(lifetime))),
				build_arg_ident(req, build_ty("RequestParams"))
			])
			.add_args(params
				.iter()
				.map(|p: &Ident| build_arg_ident(p.clone(), build_ty_ptr("str", Mutability::Immutable, Some(lifetime))))
			)
			.add_lifetime(lifetime)
			.set_return_ty(build_ty("Result<Response>"))
			.add_body_block(quote_block!(&mut cx, {
				let $qry = &$req.get_url_qry();
				let $base = &$req.base_url;
			}))
			.add_body_stmts(url_stmts)
			.add_body_block(quote_block!(&mut cx, {
				let mut headers = Headers::new();
				headers.set(ContentType::json());
			}));
			
			match *fun.method {
				HttpVerb::Head => {
					rs_fun = rs_fun
					.add_body_block(quote_block!(&mut cx, {
						let res = $client.head(&$url_ident)
							.headers(headers);
							
						res.send()
					}));
				},
				HttpVerb::Get => {
					rs_fun = rs_fun
					.add_body_block(quote_block!(&mut cx, {
						let res = $client.get(&$url_ident)
							.headers(headers);
							
						res.send()
					}));
				},
				HttpVerb::Delete => {
					rs_fun = rs_fun
					.add_body_block(quote_block!(&mut cx, {
						let res = $client.delete(&$url_ident)
							.headers(headers);
							
						res.send()
					}));
				},
				HttpVerb::Post => {
					rs_fun = rs_fun
					.add_arg(build_arg_ident(body, build_ty_ptr("str", Mutability::Immutable, Some(lifetime))))
					.add_body_block(quote_block!(&mut cx, {
						let res = $client.post(&$url_ident)
							.headers(headers)
							.body($body);
							
						res.send()
					}));
				},
				HttpVerb::Put => {
					rs_fun = rs_fun
					.add_arg(build_arg_ident(body, build_ty_ptr("str", Mutability::Immutable, Some(lifetime))))
					.add_body_block(quote_block!(&mut cx, {
						let res = $client.put(&$url_ident)
							.headers(headers)
							.body($body);
							
						res.send()
					}));
				},
				HttpVerb::Patch => {
					rs_fun = rs_fun
					.add_arg(build_arg_ident(body, build_ty_ptr("str", Mutability::Immutable, Some(lifetime))))
					.add_body_block(quote_block!(&mut cx, {
						let res = $client.patch(&$url_ident)
							.headers(headers)
							.body($body);
							
						res.send()
					}));
				}
			};
			
			try!(emitter.emit(&rs_fun, &mut src_file).map_err(|e| e.description().to_string()));
			try!(emitter.emit_str(&"\n", &mut src_file).map_err(|e| e.description().to_string()));
			try!(src_file.sync_all().map_err(|e| e.description().to_string()));
		}
		
		//6. Emit mod header if file isn't mod
		if !file_is_mod {
			let mod_path = format!("{}/{}.rs", dir_path, "mod");
			let mut mod_file = match OpenOptions::new().write(true).append(true).open(&mod_path) {
				Ok(f) => f,
				Err(_) => File::create(&mod_path).unwrap()
			};
			
			try!(emitter.emit_str(&format!("pub mod {};\n", file), &mut mod_file).map_err(|e| e.description().to_string()));
			try!(mod_file.sync_all().map_err(|e| e.description().to_string()));
		}
	}
	
	//7. Build up the mod structure
	let mut mod_paths = Vec::new();
	for entry in WalkDir::new(dest_dir).min_depth(1).max_open(1).into_iter().filter_map(|e| e.ok()) {
		let meta = entry.metadata().unwrap();
		if meta.is_dir() {
			if let Some(parent) = entry.path().parent() {
				let name = entry.file_name();
				
				mod_paths.push((
					(format!("{}/{}.rs", parent.to_str().unwrap().to_string(), "mod")), 
					name.to_str().unwrap().to_string()
				));
			}
		}
	}
	
	for (path, name) in mod_paths {
		let mut mod_file = match OpenOptions::new().write(true).append(true).open(&path) {
			Ok(f) => f,
			Err(_) => File::create(&path).unwrap()
		};
		
		try!(emitter.emit_str(&format!("pub mod {};\n", name), &mut mod_file).map_err(|e| e.description().to_string()));
		try!(mod_file.sync_all().map_err(|e| e.description().to_string()));
	}

	Ok(())
}
