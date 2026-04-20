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
        self.include.push_str(
            r#"
#include <stdint.h>
#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

#include <string.h>
#include <math.h>

typedef int32_t t_0CD; // i32
typedef void t_1CD; // [] (arr)
typedef double t_2CD; // f64
typedef bool t_3CD; // bool
typedef char t_4CD; // char
// typedef func t_5; // func - commented out as func is not a C type
typedef char* t_6CD; // strlit

t_1CD v_0s_0Ct_0CDD(t_0CD i) { // print i32
    printf("%d", i);
}

t_1CD v_0s_0Ct_2CDD(t_2CD n) { // print f64
    printf("%.6f", n);
}

t_1CD v_0s_0Ct_3CDD(t_3CD n) { // print bool
    printf("%s", n ? "true" : "false");
}

t_1CD v_0s_0Ct_6CDD(t_6CD s) { // print strlit
    printf("%s", s);
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

t_0CD v_4s_0Ct_0CDD(t_0CD n1, t_0CD n2) { // pow i32
    return pow(n1, n2);
}

t_2CD v_4s_0Ct_2CDD(t_2CD n1, t_2CD n2) { // pow f64
    return pow(n1, n2);
}

t_0CD v_5s_0Ct_0CDD(t_0CD n1, t_0CD n2) { // div i32
    return n1 / n2;
}

t_2CD v_5s_0Ct_2CDD(t_2CD n1, t_2CD n2) { // div f64
    return n1 / n2;
}

t_0CD v_6s_0CD() { // intput
    t_0CD output;
    scanf("%d", &output);
    return output;
}

t_2CD v_7s_0CD() { // fput
    t_2CD output;
    scanf("%f", &output);
    return output;
}

t_6CD v_8s_0CD() { // strput
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

t_3CD v_9s_0Ct_0CDD(t_0CD n1, t_0CD n2) { // equal i32
    return n1 == n2;
}

t_3CD v_9s_0Ct_2CDD(t_2CD n1, t_2CD n2) { // equal f64
    return n1 == n2;
}

t_3CD v_9s_0Ct_6CDD(t_6CD s1, t_6CD s2) { // equal strlit
    return strcmp(s1, s2) == 0;
}

t_3CD v_10s_0Ct_0CDD(t_0CD n1, t_0CD n2) { // greater i32
    return n1 > n2;
}

t_3CD v_10s_0Ct_2CDD(t_2CD n1, t_2CD n2) { // greater f64
    return n1 > n2;
}

t_3CD v_11s_0Ct_0CDD(t_0CD n1, t_0CD n2) { // bang_equal i32
    return n1 != n2;
}

t_3CD v_11s_0Ct_2CDD(t_2CD n1, t_2CD n2) { // bang_equal f64
    return n1 != n2;
}

t_3CD v_11s_0Ct_6CDD(t_6CD s1, t_6CD s2) { // bang_equal strlit
    return strcmp(s1, s2) != 0;
}

t_3CD v_12s_0Ct_0CDD(t_0CD n1, t_0CD n2) { // greater_equal i32
    return n1 >= n2;
}

t_3CD v_12s_0Ct_2CDD(t_2CD n1, t_2CD n2) { // greater_equal f64
    return n1 >= n2;
}

t_3CD v_13s_0Ct_0CDD(t_0CD n1, t_0CD n2) { // less_equal i32
    return n1 <= n2;
}

t_3CD v_13s_0Ct_2CDD(t_2CD n1, t_2CD n2) { // less_equal f64
    return n1 <= n2;
}

"#,
        );
        self.body.push_str(
            "
int main() {\n    ",
        );
        self.body
            .push_str(&cte.c_func_instance_name("main", &[], Span::empty()));
        self.body.push_str("();\n}");

        self.include.clone()
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
