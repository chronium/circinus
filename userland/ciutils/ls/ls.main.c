#include <dirent.h>
#include <limits.h>
#include <stdio.h>
#include <unistd.h>

int main(int argc, char **argv) {
  DIR *dir;
  struct dirent *ent;

  char cwd[PATH_MAX];
  getcwd(cwd, PATH_MAX);

  if ((dir = opendir(cwd)) != NULL) {
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