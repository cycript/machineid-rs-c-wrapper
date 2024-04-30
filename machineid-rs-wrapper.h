#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum CComponentCode_Tag
{
  SystemID,
  OSName,
  CPUCores,
  CPUID,
  DriveSerial,
  MacAddress,
  FileToken,
  Username,
  MachineName,
} CComponentCode_Tag;

typedef struct CComponentCode
{
  CComponentCode_Tag tag;
  union
  {
    struct
    {
      const char *file_token;
    };
  };
} CComponentCode;

extern "C"
{
  void setup_builder(const struct CComponentCode *component_ptr, uintptr_t len);
  char *get_machine_id(const uint8_t *secret_key);
}