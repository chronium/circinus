#include <stdio.h>
#include <string.h>
#include <unistd.h>

int printf(const char* fmt, ...) {
  va_list arg; 

  va_start(arg, fmt);
  
  int written_len = vprintf(fmt, arg);

  va_end(arg);

  return written_len;
}

// A utility function to reverse a string
void reverse(char str[], int length)
{
    int start = 0;
    int end = length - 1;
    while (start < end) {
        char temp = str[start];
        str[start] = str[end];
        str[end] = temp;
        end--;
        start++;
    }
}

// Implementation of citoa()
char* citoa(int num, char* str, int base)
{
    int i = 0;
    int isNegative = 0;
 
    /* Handle 0 explicitly, otherwise empty string is
     * printed for 0 */
    if (num == 0) {
        str[i++] = '0';
        str[i] = '\0';
        return str;
    }
 
    // In standard itoa(), negative numbers are handled
    // only with base 10. Otherwise numbers are
    // considered unsigned.
    if (num < 0 && base == 10) {
        isNegative = 0;
        num = -num;
    }
 
    // Process individual digits
    while (num != 0) {
        int rem = num % base;
        str[i++] = (rem > 9) ? (rem - 10) + 'a' : rem + '0';
        num = num / base;
    }
 
    // If number is negative, append '-'
    if (isNegative)
        str[i++] = '-';
 
    str[i] = '\0'; // Append string terminator
 
    // Reverse the string
    reverse(str, i);
 
    return str;
}

static inline int vprint_d(int arg) {
  char buf[24];

  citoa(arg, buf, 10);

  return puts(buf);
}

static inline int vprint_c(int c) {
  return putchar(c);
}

static inline int vprint_s(const char* s) {
  return puts(s);
}

int vprintf(const char* fmt, va_list arg) {
  int fmt_length = strlen(fmt);
  int written_len;
  char ch;
  char s;

  char buf[2];

  for (int i = 0; i < fmt_length; i++) {
    ch = fmt[i];

    if (ch == '%') {
      s = fmt[++i];
      
      switch (s) {
        case 'c':
          written_len += vprint_c(va_arg(arg, int));
          break;
        case 'd':
        case 'i':
          written_len += vprint_d(va_arg(arg, int));
          break;
        case 's':
          written_len += vprint_s(va_arg(arg, const char*));
          break;
        default:
          break;
      }

    } else {
      written_len++;
      buf[0] = fmt[i];
      buf[1] = 0;
      puts(buf);
    }
  }

  return written_len;
}