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
    loop {
        let mut adv_config = peripheral::Config::default();
        adv_config.timeout = Some(100); // ~1 s par cycle de diffusion

        let bthome_payload = [
            0xD2, 0xFC,        // BTHome UUID 0xFCD2 (little-endian)
            0x40,              // Device Info: version 2, non-encrypted, regular interval
            0x02, 0xC4, 0x09,  // Temperature: 25.00 °C (sint16, factor 0.01)
            0x03, 0xBF, 0x13,  // Humidity: 50.55 % (uint16, factor 0.01)
        ];

        let adv_data = LegacyAdvertisementBuilder::new()
            .flags(&[Flag::GeneralDiscovery, Flag::LE_Only])
            .full_name("DIY-sensor")
            .raw(AdvertisementDataType::SERVICE_DATA_16, &bthome_payload)
            .build();

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
