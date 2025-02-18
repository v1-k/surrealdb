use crate::api::engine::local::Db;
use crate::api::engine::local::SpeeDb;
use crate::api::err::Error;
use crate::api::opt::Config;
use crate::api::opt::Endpoint;
use crate::api::opt::IntoEndpoint;
use crate::api::opt::Strict;
use crate::api::Result;
use crate::dbs::Level;
use crate::opt::auth::Root;
use std::path::Path;
use url::Url;

impl IntoEndpoint<SpeeDb> for &str {
	type Client = Db;

	fn into_endpoint(self) -> Result<Endpoint> {
		let url = format!("speedb://{self}");
		Ok(Endpoint {
			endpoint: Url::parse(&url).map_err(|_| Error::InvalidUrl(url))?,
			config: Default::default(),
			#[cfg(any(feature = "native-tls", feature = "rustls"))]
			tls_config: None,
			auth: Level::No,
			username: String::new(),
			password: String::new(),
		})
	}
}

impl IntoEndpoint<SpeeDb> for &Path {
	type Client = Db;

	fn into_endpoint(self) -> Result<Endpoint> {
		let path = self.display().to_string();
		IntoEndpoint::<SpeeDb>::into_endpoint(path.as_str())
	}
}

impl<T> IntoEndpoint<SpeeDb> for (T, Strict)
where
	T: AsRef<Path>,
{
	type Client = Db;

	fn into_endpoint(self) -> Result<Endpoint> {
		let (path, _) = self;
		let mut endpoint = IntoEndpoint::<SpeeDb>::into_endpoint(path.as_ref())?;
		endpoint.config.strict = true;
		Ok(endpoint)
	}
}

impl<T> IntoEndpoint<SpeeDb> for (T, Config)
where
	T: AsRef<Path>,
{
	type Client = Db;

	fn into_endpoint(self) -> Result<Endpoint> {
		let (path, config) = self;
		let mut endpoint = IntoEndpoint::<SpeeDb>::into_endpoint(path.as_ref())?;
		endpoint.config = config;
		Ok(endpoint)
	}
}

impl<T> IntoEndpoint<SpeeDb> for (T, Root<'_>)
where
	T: AsRef<Path>,
{
	type Client = Db;

	fn into_endpoint(self) -> Result<Endpoint> {
		let (path, root) = self;
		let mut endpoint = IntoEndpoint::<SpeeDb>::into_endpoint(path.as_ref())?;
		endpoint.auth = Level::Kv;
		endpoint.username = root.username.to_owned();
		endpoint.password = root.password.to_owned();
		Ok(endpoint)
	}
}

impl<T> IntoEndpoint<SpeeDb> for (T, Strict, Root<'_>)
where
	T: AsRef<Path>,
{
	type Client = Db;

	fn into_endpoint(self) -> Result<Endpoint> {
		let (path, _, root) = self;
		let mut endpoint = IntoEndpoint::<SpeeDb>::into_endpoint((path.as_ref(), root))?;
		endpoint.config.strict = true;
		Ok(endpoint)
	}
}

impl<T> IntoEndpoint<SpeeDb> for (T, Config, Root<'_>)
where
	T: AsRef<Path>,
{
	type Client = Db;

	fn into_endpoint(self) -> Result<Endpoint> {
		let (path, config, root) = self;
		let mut endpoint = IntoEndpoint::<SpeeDb>::into_endpoint((path.as_ref(), root))?;
		endpoint.config = config;
		Ok(endpoint)
	}
}
