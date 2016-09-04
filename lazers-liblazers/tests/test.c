#include "../include/lazers.h"

int main (int argc, char const *argv[])
{
        CClient* c = lzrs_new_hyper_client();
        c->inspect(c->client);
        c->get(c->client, "foobar");
        c->close(c);
}
