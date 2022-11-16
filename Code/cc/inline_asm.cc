#include <iostream>

int main(){
    int oldbit = 0;
    int value = 1;

    __asm__ __volatile__("mov %1, %0" :"=r"(oldbit) : "r"(value));  // 값이동 OK
    std::cout << oldbit << std::endl;   // __volatile__은 함수가 아무런 기능 안해도 강제로 넣는다는것, 없어도 상관없지만 혹시모르니까 넣는게좋음
    // %0은 첫번째인자, %1은 두번째인자

    oldbit = 0;
    value = 1;

    __asm__ __volatile__("mov %1, %0" : : "r"(oldbit), "r"(value)); // 쓰기는 가능한데 값변경은 안된다. 읽은것을 다른데다 저장하는듯하다.
    std::cout << oldbit << std::endl;

    oldbit = 0;
    value = 1;

    __asm__ __volatile__("mov $0xffff, %0" :"=r"(oldbit) : "r"(oldbit)); // 값은 $0x0000을 통해 넣을수있다.
    std::cout << oldbit << std::endl;

    oldbit = 0;
    value = 1;

    __asm__ __volatile__("mov $0xffff, %0"
                         : "=r"(oldbit)
                         : "0"(oldbit)); // 0번째 값의 인풋을 oldbit로 할 수 있다.
    std::cout << oldbit << std::endl;

    oldbit = 0;
    unsigned long array[30];
    array[0] = 1;
    array[2] = 123;

    __asm__ __volatile__("mov %1, %0"
                         : "=r"(oldbit)
                         : "m"(array)); // m은 array가 가르키는값을 직접조작
    std::cout << oldbit << std::endl;

    array[0] = 2;

    __asm__ __volatile__("add $16, %1; mov (%1), %0"
                         : "=r"(oldbit)
                         : "r"(&array)); // m은 array가 가르키는값을 직접조작
    std::cout << oldbit << std::endl; // 성공

    array[5] = 123123;
    __asm__ __volatile__("mov 40(%1), %0"
                         : "=r"(oldbit)
                         : "r"(&array)); // m은 array가 가르키는값을 직접조작
    std::cout << oldbit << std::endl; // 성공

    __asm__ __volatile__("mov %1, %%eax; mov %%eax, %0" :"=r"(oldbit) : "r"(value) : "%eax");  // 레지스터 사용 OK

    __asm__ __volatile__("mov %1, %%eax; mov %%eax, %0"
                         : "=r"(oldbit)
                         : "r"(value)); // 레지스터 사용 OK

    long long hi = 123;
    long long hi2 = 1234;
    __asm__ __volatile__("mov %1, %%rax; mov %%rax, %0"
                         : "=r"(hi2)
                         : "r"(hi)); // 리눅스냐 윈도우냐에 따라 long 크기가 바뀌기때문에 (리눅스는 64비트, 윈도우는 32비트), rax를 쓰냐 eax를 쓰냐가 달라진다.
    return 0;
}

/*
movb %al, %bl -- Byte move
movw %ax, %bx -- Word move
movl %eax, %ebx -- Longword move
movd 가능?

=r 자리에 다음과 같은 값 사용가능
a %eax
b %ebx
c %ecx
d %edx
ra -? rax?
r은 스택에 저장된다 bp-4 bp-8등
e는 그냥 e레지스터 아무거나? 불명. 아마도 e 스택?
S %esi
D %edi
m 메모리 내부에서 직접쓰기
0은 0번째 값의 인풋지정

asm("값" : "아웃풋" : "인풋" : "값을 유지할 레지스터");
*/

/*
'm'
Architecture가 일반적으로 지원하는 모든 addressing mode중 하나를 사용하는 memory operand.

'r'
범용 레지스터 operand.

'd', 'a', 'f', ...
레지스터를 적접 지정하는 constraint. Architecture마다 다른 문자들을 정의합니다. 'd', 'a', 'f'는 68000/68020에서 쓰는 문자들입니다.

'i'
정수 immediate operand. Assemble할 때가 되어야 알 수 있는 symbol값(주소)들도 해당됩니다.

'n'
값을 정확히 알고 있는 정수. Symbol값들은 해당되지 않습니다.

'I', 'J', 'K', ... 'P'
미리 정해진 범위안의 정수 immediate operand. 예를 들어 68000에서 'I'는 1에서 8사이의 immediate operand를 뜻하며 shift operation의 shift cound로 쓰입니다.

'E'
Compling machine과 target machine의 floating 표현 방식이 같을 때 immediate floating operand를 허용합니다.

'F'
Immediate floating operand.

'G', 'H'
Machine에 따라 정의되는 특정한 범위내의 immediate floating point operand.

'g'
범용 레지스터, memory, immediate operand 중 무엇이라도 됩니다.

'X'
어떠한 operand라도 허용합니다.

'p'
유효한 주소가 허용됩니다. Load address, push address등의 instruction에 쓰입니다.

'Q', 'R', 'S', ... 'U'
Machine-dependent.



2.3.1.2. i386 specific
'q'
a, b, c, or d register

'A'
a, or d register (for 64-bit ints)

'f'
Floating point register

't'
First (top of stack) floating point register

'u'
Second floating point register

'a'
a register

'b'
b register

'c'
c register

'd'
d register

'D'
di register

'S'
si register

'I'
Constant in range 0 to 31 (for 32bit shifts)

'J'
Constant in range 0 to 63 (for 64bit shifts)

'K'
0xff

'L'
0xffff

'M'
0, 1, 2, or 3 (shifts for lea instruction)

'N'
Constant in range 0 to 255 (for out instruction)

'G'
Standard 80387 floating point constant

2.3.1.3. Modifiers
Constraint modifier들은 그 변수가 어떻게 사용되는 지를 compiler에게 알려줍니다. 아래의 리스트는 완전하지 않습니다. Gcc 메뉴얼을 참조하세요.

'='
변수의 값이 바뀜을 나타냅니다. Output들에 대해서는 이 modifier가 반드시 지정되어 있어야 합니다.

'&'
Early clobber. 다음 절에서 자세히 설명하겠습니다.

https://wiki.kldp.org/wiki.php/DocbookSgml/GCC_Inline_Assembly-KLDP
*/