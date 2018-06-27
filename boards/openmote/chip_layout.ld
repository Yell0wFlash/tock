/* Memory Space Definitions, 512K flash, 32K ram 
  //rom (rx)  : ORIGIN = 0x2048, LENGTH = 0x00020000
*/
MEMORY
{
  rom (rx) : ORIGIN = 0x00200000, LENGTH = 256K
  prog (rx) : ORIGIN = 0x00240000, LENGTH = 256K
  ccfg (rx) : ORIGIN = 0x00280000-0x0000002C, LENGTH = 0x0000002C
  ram (rwx) : ORIGIN = 0x20000000, LENGTH = 32K
}

MPU_MIN_ALIGN = 8K;
