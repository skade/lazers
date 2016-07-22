#include "../include/my_header.h"

int main (int argc, char const *argv[])
{
        lzrs_client* c = lzrs_new_hyper_client();
        lzrs_inspect_client(c);
        lzrs_close(c);
}
