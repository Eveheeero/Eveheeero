#include <iostream>

void exception(void);

int main(int argc, char **argv)
{
    exception();
    return 0;
}

void exception(void)
{
    try
    {
        // const int zero = 0;
        // int a = 1 / (zero * 2);  // can't catch
        throw std::logic_error(std::string("Invalid argument"));
    }
    catch (std::logic_error const& e)
    {
        std::cout << e.what() << std::endl;
        std::cout << sizeof(e) << std::endl;
    }
    catch(const std::exception& e)
    {
        std::cerr << e.what() << '\n';
    }
    
}