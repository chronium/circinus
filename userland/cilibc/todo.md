fputc <- putc
fputs <- puts

fprintf <- printf
fvprintf <- fprintf

fopen; fclose

streams:
  stdin line buffered if isatty otherwise fully buffered
  stdout line buffered if isatty otherwise fully buffered
  stderr not buffered

functions:
  fdopen
  isatty

  fflush
  fputc
  fwrite

