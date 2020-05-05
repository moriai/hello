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
        *(.bss);
        *(.rodata);
    }
    _heap = ALIGN(4);
}
