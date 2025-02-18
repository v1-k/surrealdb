use crate::api::engine::remote::http::Client;
use crate::api::engine::remote::http::Http;
use crate::api::engine::remote::http::Https;
use crate::api::err::Error;
use crate::api::opt::IntoEndpoint;
#[cfg(any(feature = "native-tls", feature = "rustls"))]
use crate::api::opt::Tls;
use crate::api::Endpoint;
use crate::api::Result;
use crate::dbs::Level;
use std::net::SocketAddr;
use url::Url;

impl IntoEndpoint<Http> for &str {
	type Client = Client;

	fn into_endpoint(self) -> Result<Endpoint> {
		let url = format!("http://{self}");
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

impl IntoEndpoint<Http> for SocketAddr {
	type Client = Client;

	fn into_endpoint(self) -> Result<Endpoint> {
		let url = format!("http://{self}");
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

impl IntoEndpoint<Http> for String {
	type Client = Client;

	fn into_endpoint(self) -> Result<Endpoint> {
		let url = format!("http://{self}");
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

impl IntoEndpoint<Https> for &str {
	type Client = Client;

	fn into_endpoint(self) -> Result<Endpoint> {
		let url = format!("https://{self}");
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

impl IntoEndpoint<Https> for SocketAddr {
	type Client = Client;

	fn into_endpoint(self) -> Result<Endpoint> {
		let url = format!("https://{self}");
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

impl IntoEndpoint<Https> for String {
	type Client = Client;

	fn into_endpoint(self) -> Result<Endpoint> {
		let url = format!("https://{self}");
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

#[cfg(feature = "native-tls")]
#[cfg_attr(docsrs, doc(cfg(feature = "native-tls")))]
impl<T> IntoEndpoint<Https> for (T, native_tls::TlsConnector)
where
	T: IntoEndpoint<Https>,
{
	type Client = Client;

	fn into_endpoint(self) -> Result<Endpoint> {
		let (address, config) = self;
		let mut endpoint = address.into_endpoint()?;
		endpoint.tls_config = Some(Tls::Native(config));
		Ok(endpoint)
	}
}

#[cfg(feature = "rustls")]
#[cfg_attr(docsrs, doc(cfg(feature = "rustls")))]
impl<T> IntoEndpoint<Https> for (T, rustls::ClientConfig)
where
	T: IntoEndpoint<Https>,
{
	type Client = Client;

	fn into_endpoint(self) -> Result<Endpoint> {
		let (address, config) = self;
		let mut endpoint = address.into_endpoint()?;
		endpoint.tls_config = Some(Tls::Rust(config));
		Ok(endpoint)
	}
}
