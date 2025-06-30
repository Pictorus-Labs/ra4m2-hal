MEMORY
{
  /* FLASH and RAM are mandatory memory regions */
  /* Update examples/data_overflow.rs if you change these sizes. */
  FLASH : ORIGIN = 0x00000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 128K
}