#include <stdio.h>
#include <ctype.h>
#include <string.h>

int main(int argc, char* argv[]) {
  if (argc > 1) {
    const char* word = argv[1];
    size_t len = strlen(word);
    char s[len+1];
    size_t curr_index = 0;
    for (size_t i = 0; i < len; i++) {
      if (isspace(word[i]) || word[i] == '-') {
        if (curr_index > 0 && s[curr_index-1] == '-') {
          continue;
        }
        s[curr_index] = '-';
        curr_index++;
        continue;
      }
      if (word[i] == '.') {
        s[curr_index] = '.';
        curr_index++;
        continue;
      }
      if (isalnum(word[i])) {
        s[curr_index] = tolower(word[i]);
        curr_index++;
        continue;
      }
    }
    s[curr_index] = '\0';
    puts(s);
  }
}
