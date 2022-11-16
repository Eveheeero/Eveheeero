void run();

int main(int argc, char **argv)
{
    run(); // {void (void)} 0x40157f <run()>
    // goto my; // 다른 함수에 있는 주소로는 이동할 수 없었다.
    /*
    goto는 my라는 변수에 있는 주소로 이동하는것은 아닌가? 그러면 변수에 주소값을 넣어놓으면 이동할 수 있지 않을까?
    그런것을 이용하면 다른 함수에 잇는 주소로 이동할 수 있지 않을까?
    */
    void *pointer = (void *)run;
    return 0;
}

void run()
{
    int a = 1;
my:
    int b = 2;
    if (a == 1)
    {
        a = 2;
        goto my; // jmp 0x40143f 형태로, 변수같은것으로 저장하는것이 아닌 컴파일러 단에서 설정을 해주는 듯 하다.
    }
    return;
}