/* i.MX8M Plus Cortex-M7 memory layout.
 *
 * The M7 image is loaded by U-Boot's `bootaux` into a DDR region reserved from
 * Linux (a `reserved-memory` carve-out on the A53 side). These origins/lengths
 * MUST match that carve-out and your boot configuration before running on
 * hardware. For the lowest-latency reactive path you may instead link hot code
 * into ITCM (0x00000000) and data into DTCM (0x20000000); the generous DDR
 * layout below simply lets debug builds link during bring-up.
 */
MEMORY
{
  FLASH (rx)  : ORIGIN = 0x80000000, LENGTH = 2M   /* M7 code (reserved DDR) */
  RAM   (rwx) : ORIGIN = 0x80200000, LENGTH = 1M   /* M7 data (reserved DDR) */
}
