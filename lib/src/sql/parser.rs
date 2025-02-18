use crate::err::Error;
use crate::sql::error::Error::{Field, Group, Order, Parser, Split};
use crate::sql::error::IResult;
use crate::sql::query::{query, Query};
use crate::sql::subquery::{subquery, Subquery};
use crate::sql::thing::Thing;
use crate::sql::value::Value;
use nom::Err;
use std::str;
use tracing::instrument;

/// Parses a SurrealQL [`Query`]
#[instrument(name = "parser", skip_all, fields(length = input.len()))]
pub fn parse(input: &str) -> Result<Query, Error> {
	parse_impl(input, query)
}

/// Parses a SurrealQL Subquery [`Subquery`]
#[instrument(name = "parser", skip_all, fields(length = input.len()))]
pub fn sub_query(input: &str) -> Result<Subquery, Error> {
	parse_impl(input, subquery)
}

/// Parses a SurrealQL [`Thing`]
#[instrument(name = "parser", skip_all, fields(length = input.len()))]
pub fn thing(input: &str) -> Result<Thing, Error> {
	parse_impl(input, super::thing::thing)
}

/// Parses a SurrealQL [`Value`].
#[instrument(name = "parser", skip_all, fields(length = input.len()))]
pub fn value(input: &str) -> Result<Value, Error> {
	parse_impl(input, super::value::value)
}

/// Parses JSON into an inert SurrealQL [`Value`]
#[instrument(name = "parser", skip_all, fields(length = input.len()))]
pub fn json(input: &str) -> Result<Value, Error> {
	parse_impl(input.trim(), super::value::json)
}

fn parse_impl<O>(input: &str, parser: impl Fn(&str) -> IResult<&str, O>) -> Result<O, Error> {
	// Check the length of the input
	match input.trim().len() {
		// The input query was empty
		0 => Err(Error::QueryEmpty),
		// Continue parsing the query
		_ => match parser(input) {
			// The query was parsed successfully
			Ok((v, parsed)) if v.is_empty() => Ok(parsed),
			// There was unparsed SQL remaining
			Ok((_, _)) => Err(Error::QueryRemaining),
			// There was an error when parsing the query
			Err(Err::Error(e)) | Err(Err::Failure(e)) => Err(match e {
				// There was a parsing error
				Parser(e) => {
					// Locate the parser position
					let (s, l, c) = locate(input, e);
					// Return the parser error
					Error::InvalidQuery {
						line: l,
						char: c,
						sql: s.to_string(),
					}
				}
				// There was a SPLIT ON error
				Field(e, f) => Error::InvalidField {
					line: locate(input, e).1,
					field: f,
				},
				// There was a SPLIT ON error
				Split(e, f) => Error::InvalidSplit {
					line: locate(input, e).1,
					field: f,
				},
				// There was a ORDER BY error
				Order(e, f) => Error::InvalidOrder {
					line: locate(input, e).1,
					field: f,
				},
				// There was a GROUP BY error
				Group(e, f) => Error::InvalidGroup {
					line: locate(input, e).1,
					field: f,
				},
			}),
			_ => unreachable!(),
		},
	}
}

fn truncate(s: &str, l: usize) -> &str {
	// TODO: use s.floor_char_boundary once https://github.com/rust-lang/rust/issues/93743 lands
	match s.char_indices().nth(l) {
		None => s,
		Some((i, _)) => &s[..i],
	}
}

fn locate<'a>(input: &str, tried: &'a str) -> (&'a str, usize, usize) {
	let index = input.len() - tried.len();
	let tried = truncate(tried, 100);
	let lines = input.split('\n').map(|l| l.len()).enumerate();
	let (mut total, mut chars) = (0, 0);
	for (line, size) in lines {
		total += size + 1;
		if index < total {
			let line_num = line + 1;
			let char_num = index - chars;
			return (tried, line_num, char_num);
		}
		chars += size + 1;
	}
	(tried, 0, 0)
}

#[cfg(test)]
mod tests {

	use super::*;
	use serde::Serialize;
	use std::{collections::HashMap, time::Instant};

	#[test]
	fn no_ending() {
		let sql = "SELECT * FROM test";
		let res = parse(sql);
		assert!(res.is_ok());
	}

	#[test]
	fn parse_query_string() {
		let sql = "SELECT * FROM test;";
		let res = parse(sql);
		assert!(res.is_ok());
	}

	#[test]
	fn trim_query_string() {
		let sql = "    SELECT    *    FROM    test    ;    ";
		let res = parse(sql);
		assert!(res.is_ok());
	}

	#[test]
	fn parse_complex_rubbish() {
		let sql = "    SELECT    *    FROM    test    ; /* shouldbespace */ ;;;    ";
		let res = parse(sql);
		assert!(res.is_ok());
	}

	#[test]
	fn parse_complex_failure() {
		let sql = "    SELECT    *    FROM    { }} ";
		let res = parse(sql);
		assert!(res.is_err());
	}

	#[test]
	fn parser_try() {
		let sql = "
			SELECT
				*,
				tags[$].value,
				3s as duration,
				1.345 AS number,
				test AS `some thing`,
				'2012-04-23T18:25:43.511Z' AS utctime,
				'2012-04-23T18:25:43.511-08:00' AS pacifictime,
				{ key: (3 + 1 + 2), other: 9 * 7, 'some thing': { otherkey: 'text', } } AS object
			FROM $param, test, temp, test:thingy, |test:10|, |test:1..10|
			WHERE IF true THEN 'YAY' ELSE 'OOPS' END
				AND (0.1341, 0.5719) INSIDE { type: 'Polygon', coordinates: [[[0.1341, 0.5719], [0.1341, 0.5719]]] }
				AND (3 + 3 * 4)=6
				AND 3 + 3 * 4 = 6
				AND ages CONTAINS 18
				AND if IS true
			SPLIT test.things
			VERSION '2019-01-01T08:00:00Z'
			TIMEOUT 2w;

			CREATE person SET name = 'Tobie', age += 18;
		";
		let res = parse(sql);
		assert!(res.is_ok());
		let tmp = res.unwrap();

		let enc: Vec<u8> = Vec::from(&tmp);
		let dec: Query = Query::from(enc);
		assert_eq!(tmp, dec);
	}

	#[test]
	#[cfg_attr(debug_assertions, ignore)]
	fn json_benchmark() {
		// From the top level of the repository,
		// cargo test sql::parser::tests::json_benchmark --package surrealdb --lib --release -- --nocapture --exact

		#[derive(Clone, Serialize)]
		struct Data {
			boolean: bool,
			integer: i32,
			decimal: f32,
			string: String,
			inner: Option<Box<Self>>,
			inners: Vec<Self>,
			inner_map: HashMap<String, Self>,
		}

		let inner = Data {
			boolean: true,
			integer: -1,
			decimal: 0.5,
			string: "foo".to_owned(),
			inner: None,
			inners: Vec::new(),
			inner_map: HashMap::new(),
		};
		let inners = vec![inner.clone(); 10];

		let data = Data {
			boolean: false,
			integer: 42,
			decimal: 9000.0,
			string: "SurrealDB".to_owned(),
			inner_map: inners.iter().enumerate().map(|(i, d)| (i.to_string(), d.clone())).collect(),
			inners,
			inner: Some(Box::new(inner)),
		};

		let json = serde_json::to_string(&data).unwrap();
		let json_pretty = serde_json::to_string_pretty(&data).unwrap();

		let benchmark = |de: fn(&str) -> Value| {
			let time = Instant::now();
			const ITERATIONS: u32 = 32;
			for _ in 0..ITERATIONS {
				std::hint::black_box(de(std::hint::black_box(&json)));
				std::hint::black_box(de(std::hint::black_box(&json_pretty)));
			}
			time.elapsed().as_secs_f32() / (2 * ITERATIONS) as f32
		};

		println!("sql::json took {:.10}s/iter", benchmark(|s| crate::sql::json(s).unwrap()));
	}
}
