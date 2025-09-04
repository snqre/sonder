use super::*;

pub struct PopulationConfiguration {
    pub celestial_body: Address,
    pub max_initial_count: u128,
    pub min_initial_count: u128,
    pub growth_multiplier: q::Q6<i128>
}

pub fn spawn_population(c: PopulationConfiguration) {
    use ::reliq::ops::ToPrim as _;
    
    let m_origin: Address = Address::new_from_next();
    let m_celestial_body: Address = c.celestial_body;
    let m_min_initial_count: u128 = c.min_initial_count;
    let m_max_initial_count: u128 = c.max_initial_count;
    let m_growth_multiplier: q::Q6<i128> = c.growth_multiplier;
    let mut m_count: u128 = ::fastrand::u128(c.min_initial_count..=c.max_initial_count);
    
    super::Event::on(move |event| match event {
        super::Event::Boot => vec!(super::Event::PopulationUpdate {
            origin: m_origin,
            celestial_body: m_celestial_body,
            min_initial_count: m_min_initial_count,
            max_initial_count: m_max_initial_count,
            growth_multiplier: m_growth_multiplier,
            old_count: m_count,
            new_count: m_count
        }),
        super::Event::Tick => {
            let old_count: u128 = m_count;
            let new_count: i128 = m_count.try_into().unwrap();
            let new_count: q::Q6<i128> = (new_count * 1_000000).into();
            let new_count: q::Q6<i128> = (new_count * m_growth_multiplier).unwrap();
            let new_count: u128 = new_count.to_u128().unwrap();
            m_count = new_count;
            vec!(super::Event::PopulationUpdate {
                origin: m_origin,
                celestial_body: m_celestial_body,
                min_initial_count: m_min_initial_count,
                max_initial_count: m_max_initial_count,
                growth_multiplier: m_growth_multiplier,
                old_count,
                new_count
            })
        },
        _ => vec!()
    });
}