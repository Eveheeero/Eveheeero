# 수동 컴파일 방법

1. cc(gcc) -S test.c 혹은 llc test.ll 등의 방법을 사용하여 어셈블리어를 가져온다.
2. as(gnu as) test.s -o test.o 방법을 통해 어셈블리어를 오브젝트 파일로 변환한다. (cc -c test.c와 동일한 파일이 나오게 된다.)
3. ld(gnu ld) test.o ??? ??? ??? ??? --entry main 방법을 통해 링크한다.

> - ld /lib/crti.o /lib/crtn.o /lib/crt1.o /lib/libc.so test.o -dynamic-linker /lib/ld-linux-x86-64.so.2
>
> - ld /lib/crt1.o(위 처럼 하나만 있어도 됨) /lib/libc.so test.o -dynamic-linker /lib/ld-linux-x86-64.so.2
>
> 위와 같은 명령문이 나오게 되는데, 반드시 순서를 지켜야한다. libc

이 과정 중, 어떤 상황에서든 gcc (파일명)을 통해 바로 실행파일로 변환할 수 있다. (소스코드거나, 어셈블리어거나, 오브젝트 파일이거나)

하지만 해당 변환 과정을 거치지 않고 수동 컴파일하는 방법을 알아둘 필요가 있어 보여 연구용으로 작성되었다.

[LLVM Lang](https://llvm.org/docs/LangRef.html)
[LLC](https://llvm.org/docs/CommandGuide/llc.html)
[LD](https://ftp.gnu.org/old-gnu/Manuals/ld-2.9.1/html_mono/ld.html)
[LD 한국어](http://korea.gnu.org/manual/release/ld/ld-sjp/ld-ko_3.html)

## Gnu 컴파일러 관련 명령어

- cc -E (파일명) - 전처리를 거친 소스코드 보기
- cc -S (파일명) - 어셈블리어로 변환한 소스코드 보기
- cc -c (파일명) - 링크하지 않은 컴파일 오브젝트 보기

## 링크된 라이브러리 용도

- libc - c 라이브러리, 명시해주고싶지 않으면 -lc를 적으면 된다.
- -dynamic_linker - so파일을 링킹하기 위한 링커, /lib 내부에 ld로 시작하는 so파일로 연결해주면 되는듯하다. 없으면 기본값 ld64.so가 연결되는데, 해당 파일이 없으면 오류가 나는 듯
- crt1.o, crti.o, crtn.o - Core OS C Runtime Object? C프로그램 시작지점이라는 것 같다. C프로그램의 시작지점을 담당해주는 라이브러리 (man페이지에 있는것같다.)

## gcc 컴파일 과정 보기

cc에 -v 옵션을 적용할 시 어떤 명령어가 적용되는지 알 수 있다.

## 동적링크

ld를 할 때, so를 빼고 /usr/lib/gcc/???/???/crtbegin.o, crtend.o 를 추가하면 어느정도 오류가 해결된다는것을 알아내었다.
하지만 __libc_start_main을 얻을 수 없어 추가적으로 파악하지 못하였다.
