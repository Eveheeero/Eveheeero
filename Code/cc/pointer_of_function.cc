typedef int (*functype)(void);

int
run(void);

int
main(int argc, char** argv)
{
  run();                 // call 0x401438
  functype myfunc = run; // mov 0x401438, 0xc(%esp)
  myfunc();              // mov 0xc(%esp), %eax  call [%eax]
  return 0;              // mov 0, %eax  ret
}

int
run(void)
{
  return 1;
}