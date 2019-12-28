void start(void);

asm (".code16gcc\n"
     "call  _start\n"
     "ret\n"
);
