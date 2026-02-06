#include <stdio.h>
#include <stdint.h>
#include <limits.h>

void print_binary(int32_t n, const char* desc) {
    printf("0x%08X = %11d = ", (uint32_t)n, n);
    for (int i = 31; i >= 0; i--) {
        printf("%d", (n >> i) & 1);
        if (i % 8 == 0 && i != 0) printf(" ");
    }
    printf("  %s\n", desc);
}

void demonstrate_negation_edge_cases() {
    printf("\n=== NEGATION EDGE CASES ===\n");
    
    int32_t negated = INT_MIN;
    print_binary(negated, "INT_MIN");
    print_binary(-negated, "INT_MIN negation (UNDEFINED BEHAVIOR)");

    ++negated;
    print_binary(negated, "INT_MIN+1");
    print_binary(-negated, "INT_MIN+1 negation");

    int32_t val2 = INT_MAX;
    print_binary(val2, "INT_MAX");
    print_binary(-val2, "INT_MAX negation");

    int32_t val3 = INT_MAX - 1;
    print_binary(val3, "INT_MAX-1");
    print_binary(-val3, "INT_MAX-1 negation");
}

void demonstrate_left_shift_overflow() {
    printf("\n=== LEFT SHIFT OVERFLOW (UNDEFINED BEHAVIOR) ===\n");
    
    int32_t val1 = INT_MAX / 2;
    print_binary(val1, " - starting value");
    
    int32_t result1 = val1 << 1;  // This is still defined
    print_binary(result1, "<< 1");
    
    int32_t result2 = val1 << 2;  // Result appears negative due to UNDEFINED BEHAVIOR!
    print_binary(result2, "<< 2");
    
    int32_t result3 = val1 << 3;  
    print_binary(result3, "<< 3");


    printf("\n");
    // Example: left-shifted value near edge
    int32_t val = 0x40000000;  // 2^30
    print_binary(val, " - starting value 2^30");
    
    int32_t shifted = val << 1;
    print_binary(shifted, "<< 1 (creates INT_MIN)");
    
    uint32_t u_val = 0x40000000;
    uint32_t u_shifted = u_val << 1;
    print_binary((int32_t)u_shifted, "unsigned shift << 1");
}

void demonstrate_right_shift_signed() {
    printf("\n=== RIGHT SHIFT OF NEGATIVE VALUES (IMPLEMENTATION-DEFINED) ===\n");
    
    int32_t negative = -16;
    print_binary(negative, " - starting value");
    
    for (int i = 1; i <= 6; i++) {
        int32_t shifted = negative >> i;
        char buf[32];
        sprintf(buf, ">> %d: ", i);
        print_binary(shifted, buf);
    }
}

void demonstrate_shift_by_negative_or_large() {
    printf("\n=== SHIFT BY NEGATIVE OR >= 32 (UNDEFINED BEHAVIOR) ===\n");
    
    int32_t val = 0x12345678;
    print_binary(val, " - starting value");
    
    int32_t result = val << 32;  // UNDEFINED BEHAVIOR!
    printf("==============================  UNDEFINED BEHAVIOR  ===\n");
    print_binary(result, "<< 32 (expected 0?)");

    result = val >> 32;
    printf("==============================  UNDEFINED BEHAVIOR  ===\n");
    print_binary(result, ">> 32");
    
    result = val << 33;  // UNDFEFINED BEHAVIOR!
    printf("==============================  UNDEFINED BEHAVIOR  ===\n");
    print_binary(result, "<< 33");

    result = val >> 33; 
    printf("==============================  UNDEFINED BEHAVIOR  ===\n");
    print_binary(result, ">> 33");

    result = val << -2;
    printf("==============================  UNDEFINED BEHAVIOR  ===\n");
    print_binary(result, "<< -2");
    
    result = val >> -2;
    printf("==============================  UNDEFINED BEHAVIOR  ===\n");
    print_binary(result, ">> -2");

    result = val >> 2;
    print_binary(result, ">> 2");

    result = val << 0;
    print_binary(result, "<< 0");

    result = val >> 0;
    print_binary(result, ">> 0");
}

static int16_t ixheaac_norm32(int32_t a) {
  int16_t norm_val;

  if (a == 0) {
    norm_val = 31;
  } else {
    if (a == (int32_t)0xffffffffL) {
      norm_val = 31;
    } else {
      if (a < 0) {
        a = ~a;
      }
      for (norm_val = 0; a < (int32_t)0x40000000L; norm_val++) {
        a <<= 1;
      }
    }
  }

  return norm_val;
}

void demonstrate_norm32() {
    printf("\n=== NORM32 FUNCTION ===\n");
    printf("norm32(x) = number of left shifts to get MSB into position 30\n\n");

    int32_t test_values[] = {
        0,
        -1,
        1,
        -2,
        INT_MIN,
        INT_MAX,
        0x40000000,
        0x20000000,
        0x00000100,
        0x00010000,
        0x7FFFFFFF,
        0x00000001,
        0x80000000,
    };

    int num_tests = sizeof(test_values) / sizeof(test_values[0]);

    for (int i = 0; i < num_tests; i++) {
        int32_t val = test_values[i];
        int16_t result = ixheaac_norm32(val);
        printf("norm32() ->  %2d  for ", result);
        print_binary(val, "");
    }
}

int main() {
    // printf("=================================================\n");
    // printf("CRITICAL BIT SHIFT NUANCES IN C (SIGNED 32-BIT)\n");
    // printf("=================================================\n");

    // demonstrate_negation_edge_cases();
    // demonstrate_left_shift_overflow();
    // demonstrate_right_shift_signed();
    // demonstrate_shift_by_negative_or_large();

    demonstrate_norm32();

    return 0;
}