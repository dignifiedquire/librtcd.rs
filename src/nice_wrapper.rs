use std::io;
use std::net::{ToSocketAddrs, IpAddr};

use libc::c_uint;
use nice::{self, Agent, ControllingMode, TransferMode};

pub struct IceServer {
    host: String,
    port: usize,
}

pub struct NiceConfig {
    ice_server: IceServer,
}

pub struct NiceWrapper<'a> {
    config: &'a NiceConfig,
    agent: Option<Agent>,
    stream: Option<nice::Stream<'a>>,
}

impl<'a> NiceWrapper<'a> {
    pub fn new(cfg: &'a NiceConfig) -> NiceWrapper<'a> {
        NiceWrapper {
            config: cfg,
            agent: None,
            stream: None,
        }
    }

    pub fn initialize(&self) -> io::Result<()> {
        let agent = Agent::new(TransferMode::NonReliable, ControllingMode::Server);

        // Disable upnp
        agent.disable_upnp();

        // Set up stun server config
        let stun_ip = resolve(&self.config.ice_server.host)?;
        agent.set_stun_server(stun_ip[0]);

        agent.set_stun_server_port(self.config.ice_server.port);
        // Setup callbacks

        agent.on_candidate_gathering_done(on_candidate_gathering_done);
        agent.on_reliable_transport_writable(on_reliable_transport_writable);

        let stream = agent.add_stream("application", 1, stream_callback).unwrap();

        let state = stream.get_state();
        state.wait_for(nice::NiceComponentState::NICE_COMPONENT_STATE_READY).unwrap();

        // self.agent = Some(agent);

        Ok(())
    }

    // Callback handlers
}
fn on_candidate_gathering_done(i: c_uint) -> bool {
    println!("candiate gathering done");
    true
}

fn on_reliable_transport_writable(i: c_uint, j: c_uint) {
    println!("reliable transport writable");
}

fn stream_callback(data: &[u8]) {
    println!("stream: {:?}", data);
}

/// resolve a hostname to ip address
fn resolve(host: &str) -> io::Result<Vec<IpAddr>> {
    (host, 0).to_socket_addrs().map(|iter| iter.map(|socket_address| socket_address.ip()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cfg = &NiceConfig {
            ice_server: IceServer {
                host: "stun3.l.google.com".to_string(),
                port: 19302,
            },
        };

        let a = NiceWrapper::new(cfg);
        a.initialize().unwrap();

        let b = NiceWrapper::new(cfg);
        b.initialize().unwrap();
    }
}
