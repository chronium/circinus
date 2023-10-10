// Basically a copy of Stephen Brennan's LSH
// URL: https://brennan.io/2015/01/16/write-a-shell-in-c/

#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>

int sh_help(char **args);
int sh_cd(char **args);

char *builtin_str[] = {"help", "cd"};

int (*builtin_func[])(char **) = {&sh_help, &sh_cd};

int sh_num_builtins() { return sizeof(builtin_str) / sizeof(char *); }

int sh_help(char **args) {
  printf("Circinus Shell\n");

  printf("Built in commands:\n");
  for (int i = 0; i < sh_num_builtins(); i++) {
    printf(" %s \t", builtin_str[i]);
  }
  printf("\n");

  return 1;
}

int sh_cd(char **args) {
  if (args[1] == NULL) {
    fprintf(stderr, "sh: expected argument to \"cd\"\n");
  } else {
    if (chdir(args[1]) != 0) {
      // TODO: perror("sh");
      fprintf(stderr, "sh: no such file or directory: %s\n", args[1]);
    }
  }

  return 1;
}

int sh_launch(char **args) {
  pid_t pid, wpid;
  int status;

  pid = fork();

  if (pid == 0) {
    // Child process
    if (execvp(args[0], args) == -1) {
      // TODO: perror("sh");
      fprintf(stderr, "sh: no such file or directory: %s\n", args[0]);
    }
    exit(EXIT_FAILURE);
  } else if (pid < 0) {
    // Error forking
    // TODO: perror("sh");
    fprintf(stderr, "sh: error forking\n");
  } else {
    // Parent process
    do {
      wpid = waitpid(pid, &status, WUNTRACED);
    } while (!WIFEXITED(status) /* TODO: Signals && !WIFSIGNALED(status)*/);
  }

  return 1;
}

int sh_execute(char **args) {
  int i;

  if (args == NULL) {
    return 1;
  }

  for (i = 0; i < sh_num_builtins(); i++) {
    if (strcmp(args[0], builtin_str[i]) == 0) {
      return (*builtin_func[i])(args);
    }
  }

  return sh_launch(args);
}

#define SH_RL_BUFSIZE 1024
char *sh_read_line(void) {
  int bufsize = SH_RL_BUFSIZE;
  int position = 0;
  char *buffer = malloc(sizeof(char) * bufsize);
  int c;

  if (!buffer) {
    fprintf(stderr, "sh: allocation error\n");
    exit(EXIT_FAILURE);
  }

  while (1) {
    c = getchar();

    if (c == EOF || c == '\n') {
      buffer[position] = '\0';
      return buffer;
    } else {
      buffer[position] = c;
    }
    position++;

    if (position >= bufsize) {
      bufsize += SH_RL_BUFSIZE;
      buffer = realloc(buffer, bufsize);
      if (!buffer) {
        fprintf(stderr, "sh: allocation error\n");
        exit(EXIT_FAILURE);
      }
    }
  }
}

#define SH_TOK_BUFSIZE 64
#define SH_TOK_DELIM " \t\r\n\a"
char **sh_split_line(char *line) {
  int bufsize = SH_TOK_BUFSIZE, position = 0;
  char **tokens = malloc(bufsize * sizeof(char *));
  char *token;

  if (!tokens) {
    fprintf(stderr, "sh: allocation error\n");
    exit(EXIT_FAILURE);
  }

  token = strtok(line, SH_TOK_DELIM);
  while (token != NULL) {
    tokens[position] = token;
    position++;

    if (position >= bufsize) {
      bufsize += SH_TOK_BUFSIZE;
      tokens = realloc(tokens, bufsize * sizeof(char *));
      if (!tokens) {
        fprintf(stderr, "sh: allocation error\n");
        exit(EXIT_FAILURE);
      }
    }

    token = strtok(NULL, SH_TOK_DELIM);
  }
  tokens[position] = NULL;
  return tokens;
}

#define ANSI_RESET "\x1B[0m"
#define ANSI_BLACK "\x1B[30m"
#define ANSI_RED "\x1B[31m"
#define ANSI_GREEN "\x1B[32m"
#define ANSI_YELLOW "\x1B[33m"
#define ANSI_BLUE "\x1B[34m"
#define ANSI_MAGENTA "\x1B[35m"
#define ANSI_CYAN "\x1B[36m"
#define ANSI_WHITE "\x1B[37m"

#define ANSI_BRIGHT "\x1B[1m"

void sh_loop() {
  char *line;
  char **args;
  int status;

  char *path;
  path = getenv("PATH");
  printf("PATH: %s\n", path);

  setenv("PATH", "/ext2/bin", 1);
  path = getenv("PATH");
  printf("PATH: %s\n", path);

  do {
    char cwd[PATH_MAX];
    getcwd(cwd, PATH_MAX);
    printf(ANSI_BRIGHT ANSI_BLUE "%s" ANSI_RESET, cwd);
    printf(" > ");
    fflush(NULL);
    line = sh_read_line();
    args = sh_split_line(line);
    status = sh_execute(args);

    free(line);
    free(args);
  } while (status);
}

int main(int argc, char **argv) {
  sh_loop();

  return EXIT_SUCCESS;
}
