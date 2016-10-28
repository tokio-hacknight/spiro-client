
typedef struct spiro_client spiro_client_t;

spiro_client_t *spiro_client_new();
void spiro_client_send(spiro_client_t *client, double x, double y);
