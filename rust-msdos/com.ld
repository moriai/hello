OUTPUT_FORMAT(binary)
ENTRY(_startup)
SECTIONS
{
    . = 0x0100;
    .text :
    {
        *(.startup);
        *(.text);
    }
    .data :
    {
        *(.data);
        *(.rodata);
        *(.bss);
    }
    _heap = ALIGN(4);
}
