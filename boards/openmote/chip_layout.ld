/* Memory Space Definitions, 512K flash, 32K ram 
  //rom (rx)  : ORIGIN = 0x2048, LENGTH = 0x00020000
*/
MEMORY
{
  ccfg (rw) : ORIGIN = 0x2007, LENGTH = 0x00000041
  rom (rx) : ORIGIN = 0x00200000, LENGTH = 0x00040000
  prog (rx) : ORIGIN = 0x00240000, LENGTH = 0x00040000
  ram (rwx) : ORIGIN = 0x20000000, LENGTH = 0x00008000
}

MPU_MIN_ALIGN = 4K;
