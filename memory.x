/*----deixe apenas o linker referente a sua placa e remova o resto----*/

/* Linker script for the STM32F103C8xx*/
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K
  RAM : ORIGIN = 0x20000000, LENGTH = 20K
}

/* Linker script for the STM32F103C6xx*/
/*
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 32K
  RAM : ORIGIN = 0x20000000, LENGTH = 10K
}
*/