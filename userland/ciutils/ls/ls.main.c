#include <dirent.h>
#include <limits.h>
#include <stdio.h>
#include <unistd.h>

int main(int argc, char **argv) {
  DIR *dir;
  struct dirent *ent;

  char *path;
  if (argv[1] == NULL) {
    char cwd[PATH_MAX];
    getcwd(cwd, PATH_MAX);
    path = cwd;
  } else {
    path = argv[1];
  }

  if ((dir = opendir(path)) != NULL) {
    while ((ent = readdir(dir)) != NULL) {
      printf("%s\n", ent->d_name);
    }
    closedir(dir);
  } else {
    // TODO: perror
    printf("Error opening directory\n");
    return 1;
  }

  return 0;
}