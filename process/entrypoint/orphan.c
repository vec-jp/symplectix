#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

// Creates an orphan process to be reaped. Useful for testing.
int main() {
    pid_t pid;

    if ((pid = fork()) < 0) {
        perror("could not create a child process");
        exit(1);
    }

    if (pid == 0) {
        printf("%6d: pid=%d ppid=%d\n", pid, getpid(), getppid());
        while (getppid() > 1) {
        }

        printf("%6d: pid=%d ppid=%d\n", pid, getpid(), getppid());
        sleep(15);
        return 0;
    }

    printf("%6d: pid=%d ppid=%d\n", pid, getpid(), getppid());
    sleep(1);
    return 0;
}
