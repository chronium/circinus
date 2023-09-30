// Basically a copy of Stephen Brennan's LSH
// URL: https://brennan.io/2015/01/16/write-a-shell-in-c/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int sh_help(char **args);

char *builtin_str[] = {
    "help"};

int (*builtin_func[])(char **) = {
    &sh_help};

int sh_num_builtins()
{
  return sizeof(builtin_str) / sizeof(char *);
}

int sh_help(char **args)
{
  printf("Circinus Shell\n");

  printf("Built in commands:\n");
  for (int i = 0; i < sh_num_builtins(); i++)
  {
    printf(" %s \t", builtin_str[i]);
  }
  printf("\n");

  return 1;
}

int sh_launch(char **args)
{
  return execv(args[0], args);
}

int sh_execute(char **args)
{
  int i;

  if (args == NULL)
  {
    return 1;
  }

  for (i = 0; i < sh_num_builtins(); i++)
  {
    if (strcmp(args[0], builtin_str[i]) == 0)
    {
      return (*builtin_func[i])(args);
    }
  }

  return sh_launch(args);
}

#define SH_RL_BUFSIZE 1024
char *sh_read_line(void)
{
  int bufsize = SH_RL_BUFSIZE;
  int position = 0;
  char *buffer = malloc(sizeof(char) * bufsize);
  int c;

  if (!buffer)
  {
    fprintf(stderr, "sh: allocation error\n");
    exit(EXIT_FAILURE);
  }

  while (1)
  {
    c = getchar();

    if (c == EOF || c == '\n')
    {
      buffer[position] = '\0';
      return buffer;
    }
    else
    {
      buffer[position] = c;
    }
    position++;

    if (position >= bufsize)
    {
      bufsize += SH_RL_BUFSIZE;
      buffer = realloc(buffer, bufsize);
      if (!buffer)
      {
        fprintf(stderr, "sh: allocation error\n");
        exit(EXIT_FAILURE);
      }
    }
  }
}

#define SH_TOK_BUFSIZE 64
#define SH_TOK_DELIM " \t\r\n\a"
char **sh_split_line(char *line)
{
  int bufsize = SH_TOK_BUFSIZE, position = 0;
  char **tokens = malloc(bufsize * sizeof(char *));
  char *token;

  if (!tokens)
  {
    fprintf(stderr, "sh: allocation error\n");
    exit(EXIT_FAILURE);
  }

  token = strtok(line, SH_TOK_DELIM);
  while (token != NULL)
  {
    tokens[position] = token;
    position++;

    if (position >= bufsize)
    {
      bufsize += SH_TOK_BUFSIZE;
      tokens = realloc(tokens, bufsize * sizeof(char *));
      if (!tokens)
      {
        fprintf(stderr, "sh: allocation error\n");
        exit(EXIT_FAILURE);
      }
    }

    token = strtok(NULL, SH_TOK_DELIM);
  }
  tokens[position] = NULL;
  return tokens;
}

void sh_loop()
{
  char *line;
  char **args;
  int status;

  do
  {
    printf("> ");
    fflush(NULL);
    line = sh_read_line();
    args = sh_split_line(line);
    status = sh_execute(args);

    free(line);
    free(args);
  } while (status);
}

int main(int argc, char **argv)
{
  sh_loop();

  return EXIT_SUCCESS;
}
