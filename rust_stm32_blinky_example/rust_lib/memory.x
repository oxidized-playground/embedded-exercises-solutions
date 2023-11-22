/* memory.x - Linker script for the STM32F302R8 */
MEMORY
{
  /* Flash memory begins at 0x80000000 and has a size of 64kB*/
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K
  /* RAM begins at 0x20000000 and has a size of 16kB*/
  RAM : ORIGIN = 0x20000000, LENGTH = 16K
}