use std::{
    borrow::Cow,
    fmt,
    net::{IpAddr, SocketAddr},
};

fn main() {
    println!("Refactor me!");

    let mut err = Error::new("NO_USER");
    err.status(404).message("User not found");
}

#[derive(Debug)]
pub struct Error<'code, 'msg> {
    code: Cow<'code, str>,
    status: u16,
    message: Cow<'msg, str>,
}

impl Default for Error<'_, '_> {
    #[inline]
    fn default() -> Self {
        Self {
            code: "UNKNOWN".into(),
            status: 500,
            message: "Unknown error has happened.".into(),
        }
    }
}

impl<'code, 'msg> Error<'code, 'msg> {
    pub fn new(code: impl Into<Cow<'code, str>>) -> Self {
        let code = code.into();
        Self {
            code,
            ..Self::default()
        }
    }

    pub fn status(&mut self, s: u16) -> &mut Self {
        self.status = s;
        self
    }

    pub fn message(&mut self, m: impl Into<Cow<'msg, str>>) -> &mut Self {
        self.message = m.into();
        self
    }
}

#[derive(Debug)]
pub struct Server(SocketAddr);

impl Server {
    pub fn bind(addr: impl Into<IpAddr>, port: u16) -> Self {
        Self(SocketAddr::from((addr.into(), port)))
    }
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[cfg(test)]
mod server_spec {
    use super::*;

    mod bind {
        use std::net::Ipv4Addr;

        use super::*;

        #[test]
        fn sets_provided_address_to_server() {
            let server = Server::bind(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
            assert_eq!(format!("{}", server), "127.0.0.1:8080");

            let server = Server::bind("::1".parse::<IpAddr>().unwrap(), 9911);
            assert_eq!(format!("{}", server), "[::1]:9911");
        }
    }
}
