int add(int a, int b) { // cpp타입의 라이브러리를 만들기 위해선 실행할 함수에 extern "C" { }를 통해 랩 해주어야 한다.
    return a + b;
}
/*
dll을 mingw를 이용해 만드는법은 dlltool 혹은 dllwrap의 사용이 필요하다.
dllwrap은 dlltool이 여러번에 걸쳐 작업하는걸 한번에 하게 해주는것인데 콘솔전용이라 잘 동작하지 않는다?

먼저, DLL을 구성하는 헤더 파일과 소스 파일이 있을때, 이들을 컴파일합니다. 이들을 링크하지 않고, 컴파일만 합니다. 옵션은 -c로서, 그 파일 이름이 create.c라고 한다면 다음과 같습니다.
gcc -c create.c -o create.o

object 파일이 생성되었으므로, def 파일을 생성합니다.
dlltool --output-def create.def --kill-at --dllname create.dll create.o

def 파일이 생성되었으면, 라이브러리를 생성합니다. 이것은 DLL이 아니라, DLL을 생성할 때 필요한 코드를 참조할 라이브러리입니다.
dlltool --output-lib libcreate.a --input-def create.def --kill-at --dllname create.dll create.o

이제 생성한 파일들을 하나로 묶어서 DLL을 작성하는 일만 남았습니다. 하나로 묶는 작업은 gcc를 사용합니다.
gcc create.o -o create.dll -mwindows -Wall -L. -lcreate -shared

마지막의 -shared 옵션은, 이 파일은 다른 실행 가능한 파일과 연관되어 있다는 것을 컴파일러에게 알려주는 옵션입니다. 링크와 관련된 옵션입니다. 
이 옵션을 포함하지 않는다면, WinMain이 없다는 에러메세지를 받게 될 것입니다.
이제 DLL이 생성되고 이 파일을 필요로 하는 곳에 복사하거나, path로 지정되어 있는 디렉토리에 옮겨두면 Windows는 이 DLL이 필요할 때 사용할 수 있습니다.
*/