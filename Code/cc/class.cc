class myClass{
    int value;
public:
    void set(int value){ this->value = value; }
    int get(){ return this->value; }
};

int main(int argc, char **argv)
{
    myClass obj;
    obj.set(42);
    obj.get();
    return 0;
}

/*
Dump of assembler code for function main(int, char**):
   0x00007ff6b4ca17d0 <+0>:	push   %rbp
   0x00007ff6b4ca17d1 <+1>:	mov    %rsp,%rbp
   0x00007ff6b4ca17d4 <+4>:	sub    $0x30,%rsp
   0x00007ff6b4ca17d8 <+8>:	mov    %ecx,0x10(%rbp)
   0x00007ff6b4ca17db <+11>:	mov    %rdx,0x18(%rbp)
   0x00007ff6b4ca17df <+15>:	call   0x7ff6b4ca18c7 <__main>
=> 0x00007ff6b4ca17e4 <+20>:	lea    -0x4(%rbp),%rax  // rbp - 4에 class를 저장한다
   0x00007ff6b4ca17e8 <+24>:	mov    $0x2a,%edx   // 42
   0x00007ff6b4ca17ed <+29>:	mov    %rax,%rcx    // 주소는 rcx에 들어간다.
   0x00007ff6b4ca17f0 <+32>:	call   0x7ff6b4ca3060 <_ZN7myClass3setEi>   // 클래스 메소드를 호출한다 (클래스의 주소를 보내면서?)
   0x00007ff6b4ca17f5 <+37>:	lea    -0x4(%rbp),%rax
   0x00007ff6b4ca17f9 <+41>:	mov    %rax,%rcx    // 다시 rcx에 클래스의 주소를 준 다음 get를 호출한다. 변수 혹은 클래스의 주소만 저장되는듯하다.
   0x00007ff6b4ca17fc <+44>:	call   0x7ff6b4ca3050 <_ZN7myClass3getEv>
   0x00007ff6b4ca1801 <+49>:	mov    $0x0,%eax
   0x00007ff6b4ca1806 <+54>:	add    $0x30,%rsp
   0x00007ff6b4ca180a <+58>:	pop    %rbp
   0x00007ff6b4ca180b <+59>:	ret    
*/