use crate::key::database::tb;
use crate::key::database::tb::Tb;
use crate::sql::statements::DefineTableStatement;

#[tokio::test]
#[serial]
async fn table_definitions_can_be_scanned() {
	// Setup
	let test = match init().await {
		Ok(ctx) => ctx,
		Err(e) => panic!("{:?}", e),
	};
	let mut tx = match test.db.transaction(true, false).await {
		Ok(tx) => tx,
		Err(e) => panic!("{:?}", e),
	};

	// Create a table definition
	let namespace = "test_namespace";
	let database = "test_database";
	let table = "test_table";
	let key = Tb::new(namespace, database, table);
	let value = DefineTableStatement {
		name: Default::default(),
		drop: false,
		full: false,
		view: None,
		permissions: Default::default(),
		changefeed: None,
	};
	match tx.set(&key, &value).await {
		Ok(_) => {}
		Err(e) => panic!("{:?}", e),
	};

	// Validate with scan
	match tx.scan(tb::prefix(namespace, database)..tb::suffix(namespace, database), 1000).await {
		Ok(scan) => {
			assert_eq!(scan.len(), 1);
			let read = DefineTableStatement::from(&scan[0].1);
			assert_eq!(&read, &value);
		}
		Err(e) => panic!("{:?}", e),
	}
}

#[tokio::test]
#[serial]
async fn table_definitions_can_be_deleted() {
	// Setup
	let test = match init().await {
		Ok(ctx) => ctx,
		Err(e) => panic!("{:?}", e),
	};
	let mut tx = match test.db.transaction(true, false).await {
		Ok(tx) => tx,
		Err(e) => panic!("{:?}", e),
	};

	// Create a table definition
	let namespace = "test_namespace";
	let database = "test_database";
	let table = "test_table";
	let key = Tb::new(namespace, database, table);
	let value = DefineTableStatement {
		name: Default::default(),
		drop: false,
		full: false,
		view: None,
		permissions: Default::default(),
		changefeed: None,
	};
	match tx.set(&key, &value).await {
		Ok(_) => {}
		Err(e) => panic!("{:?}", e),
	};

	// Validate delete
	match tx.del(&key).await {
		Ok(_) => {}
		Err(e) => panic!("{:?}", e),
	};

	// Should not exist
	match tx.get(&key).await {
		Ok(None) => {}
		Ok(Some(o)) => panic!("Should not exist but was {:?}", o),
		Err(e) => panic!("Unexpected error on get {:?}", e),
	};
}
