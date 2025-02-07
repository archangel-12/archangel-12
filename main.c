// this c code is inspired by this repo: //
// https://github.com/liununu/liununu //
// please leave him a star and hopefully if you see this, umm, well, gimme a star too („• ֊ •„) //
// truth to be told, i just don't know why i use c when it can be done in javascript :/ //
// but anyways, this is my implementation: //

#include <stdio.h>
#include <time.h>
#include <stdlib.h>
#include <string.h>

#define README_PATH "README.md"

int main() {
    time_t now;
    time(&now);

    struct tm start_tm = {0}, end_tm = {0};
    struct tm *current_tm = localtime(&now);
    int thisYear = current_tm->tm_year + 1900;

    start_tm.tm_year = thisYear - 1900;
    start_tm.tm_mon = 0;
    start_tm.tm_mday = 1;
    time_t start_time = mktime(&start_tm);

    end_tm.tm_year = thisYear - 1900;
    end_tm.tm_mon = 11;
    end_tm.tm_mday = 31;
    end_tm.tm_hour = 23;
    end_tm.tm_min = 59;
    end_tm.tm_sec = 59;
    time_t end_time = mktime(&end_tm);

    double progress_of_this_year = (double)(now - start_time) / (end_time - start_time) * 100.0;
    printf("Progress: %.2f%%\n", progress_of_this_year);

    FILE *file = fopen(README_PATH, "r");
    if (!file) {
        perror("Error: README.md not found :(");
        return 1;
    }

    fseek(file, 0, SEEK_END);
    size_t length = ftell(file);
    fseek(file, 0, SEEK_SET);

    char *content = malloc(length + 1);
    if (!content) {
        perror("Memory allocation error");
        fclose(file);
        return 1;
    }

    fread(content, 1, length, file);
    content[length] = '\0';
    fclose(file);

    file = fopen(README_PATH, "w");
    if (!file) {
        perror("Error opening README.md for writing");
        free(content);
        return 1;
    }

    char *saveptr;
    char *line = strtok_r(content, "\n", &saveptr);
    while (line) {
        if (strncmp(line, "- just so you know,", 19) == 0) {
            fprintf(file, "- just so you know, %d is %.2f%% complete\n\n", thisYear, progress_of_this_year);
        } else {
            fprintf(file, "%s\n", line);
        }
        line = strtok_r(NULL, "\n", &saveptr);
    }

    fprintf(file, "\n");

    fflush(file);
    fclose(file);
    free(content);

    return 0;
}