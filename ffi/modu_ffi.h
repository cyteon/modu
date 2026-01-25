#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum FFIType {
  Null,
  String,
  Integer,
  Float,
  Boolean,
} FFIType;

typedef union FFIValueUnion {
  char *string;
  int64_t integer;
  double float;
  bool boolean;
} FFIValueUnion;

typedef struct FFIValue {
  enum FFIType ty;
  union FFIValueUnion value;
} FFIValue;

void ffi_free_string(char *ptr);
