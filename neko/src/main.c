#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
 
#include <chibi/eval.h>
#include <chibi/sexp.h>

#include <nng/nng.h>
#include <nng/protocol/bus0/bus.h>

void fatal(const char *func, int rv) {
    fprintf(stderr, "%s: %s\n", func, nng_strerror(rv));
    exit(1);
}

void load_script(sexp ctx, char* str) {
    sexp_gc_var2(obj1, obj2);
    sexp_gc_preserve2(ctx, obj1, obj2);

    obj1 = sexp_c_string(ctx,str, -1);
    sexp_load(ctx, obj1, NULL);
  
    sexp_gc_release2(ctx);
}

void eval_string(sexp ctx, char* str) {
    sexp_gc_var2(obj1, obj2);
    sexp_gc_preserve2(ctx, obj1, obj2);

    sexp_eval_string(ctx,str, -1, NULL);
  
    sexp_gc_release2(ctx);
}

void recieve_sexp(nng_socket sock) {
    char *buf = NULL;
    size_t sz;
    nng_recv(sock, &buf, &sz, NNG_FLAG_ALLOC);
    printf("接收：%s\n", buf);
    printf("> ");
    nng_free(buf, sz);
}

void send_sexp(nng_socket sock, char *arg) {
    // SEND
    int rv;
    size_t sz;
    sz = strlen(arg);
    printf("发送完毕\n");
    rv = nng_send(sock, arg, sz, 0);
    if (rv != 0) {
	fprintf(stderr, "发送错误 error: %s\n", nng_strerror(rv));
	return;
    }
}

sexp init_sexp() {
    sexp ctx;
    sexp_scheme_init();
    ctx = sexp_make_eval_context(NULL, NULL, NULL, 0, 0);
    sexp_load_standard_env(ctx, NULL, SEXP_SEVEN);
    sexp_load_standard_ports(ctx, NULL, stdin, stdout, stderr, 1);
    load_script(ctx,"./boopinfo.neko");
    return ctx;
}

nng_socket init_node(int argc, char **argv) {
    nng_socket sock;
    int rv;
    size_t sz;
 
    if ((rv = nng_bus0_open(&sock)) != 0) {
	fatal("nng_bus0_open", rv);
    }
    if ((rv = nng_listen(sock, argv[2], NULL, 0)) != 0) {
	fatal("nng_listen", rv);
    }
    
    if (argc >= 3) {
	for (int x = 3; x < argc; x++) {
	    nng_dial(sock, argv[x], NULL, NNG_FLAG_NONBLOCK);
	}
    }
    
    sleep(1); //等待连接建立
    return sock;
}

void repl_loop(sexp ctx, nng_socket *sock) {
    char input[1024];
    
    for (;;) {
	printf("> ");
	fflush(stdout);

	if (!fgets(input, sizeof(input), stdin)) {
	    // Ctrl+D（EOF），我们不退出，只是跳过
	    clearerr(stdin);  // 清除 EOF 标志，避免卡住
	    continue;         // 不执行，下一轮循环
        }
	
	if (strncmp(input, "(exit)", 6) == 0) break;

        if (sock != NULL) {
	    send_sexp(*sock,input);
        }
	
	sexp result = sexp_eval_string(ctx, input, -1, NULL);
	
	if (sexp_exceptionp(result)) {
	    sexp_print_exception(ctx, result, sexp_current_error_port(ctx));
	} else {
	    sexp_write(ctx, result, sexp_current_output_port(ctx));
	    sexp_newline(ctx,sexp_current_output_port(ctx));
	}
    }
}

void *recv_loop(void *arg) {
    nng_socket sock = *(nng_socket*)arg;
    for (;;) {      
	recieve_sexp(sock);
    }
}

int main(int argc, char **argv) {
    sexp ctx = init_sexp();
    nng_socket *sock = NULL;
    nng_socket node;
    if (argc >= 2) {
	node = init_node(argc, argv);
	sock = &node;
	if (sock != NULL) {
	    pthread_t recv_thread;
	    if (pthread_create(&recv_thread, NULL, recv_loop, sock) != 0) {
		perror("pthread_create");
	    }
	}
    }
    repl_loop(ctx, sock);
    
    // 退出程序：
    if (sock != NULL) {
	nng_close(*sock);
    }
    sexp_destroy_context(ctx);
    return 0;
}
