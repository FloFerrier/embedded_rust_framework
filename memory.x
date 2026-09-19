MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* NRF52833 (micro:bit v2) with SoftDevice S140 7.3.0 */
  FLASH : ORIGIN = 0x00000000 + 156K, LENGTH = 512K - 156K
  RAM   : ORIGIN = 0x20000000 + 0x3338, LENGTH = 128K - 0x3338
}
