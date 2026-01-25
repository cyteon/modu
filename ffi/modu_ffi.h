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

typedef struct FFIValueUnion {
  char *string;
  int64_t integer;
  double float_;
  bool boolean;
} FFIValueUnion;

typedef struct FFIValue {
  enum FFIType ty;
  struct FFIValueUnion value;
} FFIValue;

void ffi_free_string(char *ptr);
