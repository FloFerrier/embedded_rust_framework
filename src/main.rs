#![no_std]
#![no_main]

use defmt::{info, unwrap};
use defmt_rtt as _;
use panic_probe as _;
use embassy_executor::Spawner;
use embassy_nrf::interrupt;
use nrf_softdevice::ble::advertisement_builder::{
    AdvertisementDataType, Flag, LegacyAdvertisementBuilder, LegacyAdvertisementPayload,
};
use nrf_softdevice::ble::peripheral::{self, AdvertiseError, NonconnectableAdvertisement};
use nrf_softdevice::{raw, Softdevice};

/// Trame de données simulée (base) — les 4 premiers octets sont remplacés
/// par un compteur de séquence 32 bits (little-endian) à chaque cycle.
static FRAME_TEMPLATE: [u8; 12] =
    [0xDE, 0xAD, 0xBE, 0xEF, 0xC0, 0xFF, 0xEE, 0x42, 0x13, 0x37, 0x55, 0xAA];

const COMPANY_ID: u16 = 0x0059; // Nordic Semiconductor ASA

#[embassy_executor::task]
async fn softdevice_task(sd: &'static Softdevice) -> ! {
    sd.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = interrupt::Priority::P2;
    config.time_interrupt_priority = interrupt::Priority::P2;
    let _p = embassy_nrf::init(config);
    info!("embassy init ok");

    // micro:bit v2 : horloge LF interne RC (pas de cristal 32 kHz dédié)
    let config = nrf_softdevice::Config {
        clock: Some(raw::nrf_clock_lf_cfg_t {
            source: raw::NRF_CLOCK_LF_SRC_RC as u8,
            rc_ctiv: 16,
            rc_temp_ctiv: 2,
            accuracy: raw::NRF_CLOCK_LF_ACCURACY_500_PPM as u8,
        }),
        ..Default::default()
    };

    let sd = Softdevice::enable(&config);
    info!("softdevice enabled");
    unwrap!(spawner.spawn(softdevice_task(sd)));
    info!("sd task spawned");

    // Scan response : nom complet (statique)
    static SCAN_DATA: LegacyAdvertisementPayload = LegacyAdvertisementBuilder::new()
        .full_name("MB2-POC")
        .build();

    info!("advertising MB2-POC");
    let mut seq: u32 = 0;
    loop {
        let mut adv_config = peripheral::Config::default();
        adv_config.timeout = Some(100); // ~1 s par cycle de diffusion

        // Company ID (little-endian) + trame : compteur u32 puis base statique
        let mut mfg = [0u8; 2 + FRAME_TEMPLATE.len()];
        mfg[0] = COMPANY_ID as u8;
        mfg[1] = (COMPANY_ID >> 8) as u8;
        mfg[2..6].copy_from_slice(&seq.to_le_bytes());
        mfg[6..].copy_from_slice(&FRAME_TEMPLATE[4..]);

        // Payload legacy reconstruit à chaque cycle (max 31 o) :
        // flags(3) + mfg data(2+2+12=16) = 19 o
        let adv_data = LegacyAdvertisementBuilder::new()
            .flags(&[Flag::GeneralDiscovery, Flag::LE_Only])
            .raw(AdvertisementDataType::MANUFACTURER_SPECIFIC_DATA, &mfg)
            .build();
        seq = seq.wrapping_add(1);

        match peripheral::advertise(
            sd,
            NonconnectableAdvertisement::ScannableUndirected {
                adv_data: &adv_data,
                scan_data: &SCAN_DATA,
            },
            &adv_config,
        )
        .await
        {
            Ok(()) => info!("adv cycle ok"),
            Err(AdvertiseError::Timeout) => {} // cycle terminé -> rediffusion
            Err(e) => info!("adv error: {:?}", e),
        }
    }
}
