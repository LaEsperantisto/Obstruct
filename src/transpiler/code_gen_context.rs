use crate::span::Span;
use crate::transpiler::compiletime_env::CompileTimeEnv;

pub struct CodeGenContext {
    pub include: String,
    pub types: String,
    pub declarations: String,
    pub unnamed: String,
    pub body: String,
}

impl CodeGenContext {
    /// The constructor for CodeGenContext
    pub fn new() -> CodeGenContext {
        Self::new_empty()
    }

    pub fn new_empty() -> CodeGenContext {
        CodeGenContext {
            include: String::new(),
            types: String::new(),
            body: String::new(),
            declarations: String::new(),
            unnamed: String::new(),
        }
    }

    /// Combines all the parts of the variable into one single String.
    pub fn combine(&mut self, cte: &mut CompileTimeEnv) -> String {
        let base_include = r#"
#include <stdint.h>
#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

#include <string.h>
#include <math.h>
#include <sys/ioctl.h>
#include <unistd.h>
#include <termios.h>
#include <fcntl.h>

typedef int32_t t_0CD; // i32
typedef void t_1CD; // [] (arr)
typedef double t_2CD; // f64
typedef bool t_3CD; // bool
typedef char t_4CD; // char
// typedef func t_5; // func - commented out as func is not a C type
typedef char* t_6CD; // strlit
// typedef *t_7; // ref - commented out as refs have to be "of" another type

"#;

        let base_body = r#"
static struct termios G_orig_term;
static int G_input_disabled = 0;

t_1CD v_0s_0Ct_0CDD(t_0CD i) { // print i32
    printf("%d", i);
    fflush(stdout);
}

t_1CD v_0s_0Ct_2CDD(t_2CD n) { // print f64
    printf("%.6f", n);
    fflush(stdout);
}

t_1CD v_0s_0Ct_3CDD(t_3CD n) { // print bool
    printf("%s", n ? "true" : "false");
    fflush(stdout);
}

t_1CD v_0s_0Ct_6CDD(t_6CD s) { // print strlit
    printf("%s", s);
    fflush(stdout);
}

t_0CD v_1s_0Ct_0CDD(t_0CD n1, t_0CD n2) { // add i32
    return n1 + n2;
}

t_2CD v_1s_0Ct_2CDD(t_2CD n1, t_2CD n2) { // add f64
    return n1 + n2;
}

t_6CD v_1s_0Ct_6CDD(t_6CD a, t_6CD b) { // add str
    size_t len_a = strlen(a);
    size_t len_b = strlen(b);

    char* result = malloc(len_a + len_b + 1);
    if (!result) return NULL;

    memcpy(result, a, len_a);

    memcpy(result + len_a, b, len_b);

    result[len_a + len_b] = '\0';

    return result;
}

static char _catbuf[256];
t_6CD v_1s_0Ct_6Ct_4CDD(t_6CD a, t_4CD b) { // add strlit+char
    size_t len = strlen(a);
    _catbuf[0] = b;
    _catbuf[1] = '\0';
    char* result = malloc(len + 2);
    if (!result) return NULL;
    strcpy(result, a);
    strcat(result, _catbuf);
    return result;
}


t_0CD v_2s_0Ct_0CDD(t_0CD n1, t_0CD n2) { // less i32
    return n1 < n2;
}

t_2CD v_2s_0Ct_2CDD(t_2CD n1, t_2CD n2) { // less f64
    return n1 < n2;
}

t_0CD v_3s_0Ct_0CDD(t_0CD n1, t_0CD n2) { // sub i32
    return n1 - n2;
}

t_2CD v_3s_0Ct_2CDD(t_2CD n1, t_2CD n2) { // sub f64
    return n1 - n2;
}

t_0CD v_4s_0Ct_0CDD(t_0CD n1, t_0CD n2) { // mult i32
    return n1 * n2;
}

t_2CD v_4s_0Ct_2CDD(t_2CD n1, t_2CD n2) { // mult f64
    return n1 * n2;
}

t_0CD v_5s_0Ct_0CDD(t_0CD n1, t_0CD n2) { // pow i32
    return pow(n1, n2);
}

t_2CD v_5s_0Ct_2CDD(t_2CD n1, t_2CD n2) { // pow f64
    return pow(n1, n2);
}

t_0CD v_6s_0Ct_0CDD(t_0CD n1, t_0CD n2) { // div i32
    return n1 / n2;
}

t_2CD v_6s_0Ct_2CDD(t_2CD n1, t_2CD n2) { // div f64
    return n1 / n2;
}

t_0CD v_7s_0CD() { // intput
    t_0CD output;
    scanf("%d", &output);
    return output;
}

t_2CD v_8s_0CD() { // fput
    t_2CD output;
    scanf("%f", &output);
    return output;
}

t_6CD v_9s_0CD() { // strput
    char* buffer = malloc(256 * sizeof(char));
    if (buffer == NULL) return NULL; // Handle allocation failure

    if (fgets(buffer, 256, stdin)) {
        // Optional: Remove trailing newline
        buffer[strcspn(buffer, "\n")] = 0;
        return buffer;
    }

    free(buffer);
    return NULL;
}

t_3CD v_10s_0Ct_0CDD(t_0CD n1, t_0CD n2) { // equal i32
    return n1 == n2;
}

t_3CD v_10s_0Ct_2CDD(t_2CD n1, t_2CD n2) { // equal f64
    return n1 == n2;
}

t_3CD v_10s_0Ct_6CDD(t_6CD s1, t_6CD s2) { // equal strlit
    return strcmp(s1, s2) == 0;
}

t_3CD v_10s_0Ct_4CDD(t_4CD n1, t_4CD n2) { // equal char
    return n1 == n2;
}

t_3CD v_11s_0Ct_0CDD(t_0CD n1, t_0CD n2) { // greater i32
    return n1 > n2;
}

t_3CD v_11s_0Ct_2CDD(t_2CD n1, t_2CD n2) { // greater f64
    return n1 > n2;
}

t_3CD v_12s_0Ct_0CDD(t_0CD n1, t_0CD n2) { // bang_equal i32
    return n1 != n2;
}

t_3CD v_12s_0Ct_4CDD(t_4CD n1, t_4CD n2) { // bang_equal char
    return n1 != n2;
}

t_3CD v_12s_0Ct_2CDD(t_2CD n1, t_2CD n2) { // bang_equal f64
    return n1 != n2;
}

t_3CD v_12s_0Ct_6CDD(t_6CD s1, t_6CD s2) { // bang_equal strlit
    return strcmp(s1, s2) != 0;
}

t_3CD v_13s_0Ct_0CDD(t_0CD n1, t_0CD n2) { // greater_equal i32
    return n1 >= n2;
}

t_3CD v_13s_0Ct_4CDD(t_4CD n1, t_4CD n2) { // greater_equal char
    return n1 >= n2;
}

t_3CD v_13s_0Ct_2CDD(t_2CD n1, t_2CD n2) { // greater_equal f64
    return n1 >= n2;
}

t_3CD v_14s_0Ct_0CDD(t_0CD n1, t_0CD n2) { // less_equal i32
    return n1 <= n2;
}

t_3CD v_14s_0Ct_2CDD(t_2CD n1, t_2CD n2) { // less_equal f64
    return n1 <= n2;
}

t_3CD v_15s_0CD(t_3CD b1, t_3CD b2) { // or
    return b1 || b2;
}

t_3CD v_16s_0CD(t_3CD b1, t_3CD b2) { // and
    return b1 && b2;
}

t_3CD v_17s_0CD(t_3CD b) { // not
    return !b;
}

t_0CD v_18s_0CD() { // terminal_width
    struct winsize w;

    if (ioctl(STDOUT_FILENO, TIOCGWINSZ, &w) == -1) {
        return -1;
    }

    return w.ws_col;
}

t_0CD v_19s_0CD() { // terminal_height
    struct winsize w;

    if (ioctl(STDOUT_FILENO, TIOCGWINSZ, &w) == -1) {
        return -1;
    }

    return w.ws_row;
}

t_4CD v_20s_0CD() { // get_char
    char ch = getchar();

    return ch;
}

t_1CD v_21s_0CD(t_0CD time) { // sleep
    usleep(time);
}

"#;

        let include = base_include.to_string() + base_body;

        self.body.push_str(
            "
int main() {\n    ",
        );
        self.body
            .push_str(&cte.c_func_instance_name("main", &[], Span::empty()));
        self.body.push_str("();\n}");

        include
            + "\n\n"
            + &self.types
            + "\n\n"
            + &self.declarations
            + "\n\n"
            + &self.unnamed
            + "\n\n"
            + &self.body
    }
}
