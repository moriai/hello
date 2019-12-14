void start(void);

asm (".code16gcc\n"
     "call  _start\n"
     "mov   $0x4c,%ah\n"
     "int   $0x21\n");
