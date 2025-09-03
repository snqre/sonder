use super::*;

static SPRITE_URLS: [Asset; 8] = [
    asset!("asset/location/galaxy-0.gif"),
    asset!("asset/location/galaxy-1.gif"),
    asset!("asset/location/galaxy-2.gif"),
    asset!("asset/location/galaxy-3.gif"),
    asset!("asset/location/galaxy-4.gif"),
    asset!("asset/location/galaxy-5.gif"),
    asset!("asset/location/galaxy-6.gif"),
    asset!("asset/location/galaxy-7.gif")
];

pub fn spawn_galaxy() {
    let m_port: u128 = 0;
    let m_sprite_urls_len: usize = SPRITE_URLS.len();
    let m_sprite_url: Asset = SPRITE_URLS[::fastrand::usize(0..m_sprite_urls_len)];
    
    on(Box::new(move |event| {
        match event {
            super::Event::Boot => post(super::Event::GalaxySpawn {
                port: m_port,
                sprite_url: m_sprite_url
            }),
            _ => {}
        }
    }));

    for _ in 0..=8 {
        spawn_celestial_body(None);
    }
}