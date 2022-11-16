g++ -c ASM.cpp -o ASM.out
g++ -S ASM.cpp -o ASM.S
as *.s -o s.out
g++ ASM.out s.out
./a.out