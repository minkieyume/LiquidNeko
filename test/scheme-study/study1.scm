(begin
  (display "Hello, World!")
  (newline))
(vector 0 1 2 3 4)
(cons 1 #t)
(define name "Neko")
(define (greet name)
  (string-append "My " "name " "is " name))
(greet name)
;; let赋予临时环境
(let ((name "Chiko") (neko "Nyas"))
  (display (string-append name neko)))
;; Let*允许绑定中的表达式调用前一个表达式中的值
(let* ((name "Chiko")
        (neko
          (string-append name " Nya")))
  (display neko))
(define (neko-add neko-name . nums)
  (string-append neko-name (number->string (apply + nums))))
(neko-add "neko " 1 2 3 4 5)
(define (neeko x y)
  (values (+ x y)
          (- x y)))
(neeko 1 2)
(define (silk-cat n1 n2)
  (cond
    ((< n1 n2)
      "n1小于n2")
    ((> n1 n2)
      "n1大于n2")
    (else
      "n1等于n2")))
(silk-cat 1 2)
(silk-cat 2 1)
(silk-cat 2 2)

;; 闭包
(define (make-silk-cat n1 n2)
  (define (silk-cat n)
    (cond
      ((< n n1)
        "n小于n1")
      ((> n n2)
        "n大于n2")
      (else
        "n为别的值")))
  silk-cat)
(make-silk-cat 1 2)

;; 递归
(define (build-tree depth)
  (if (= depth 0)
    '(0)
    (list depth
      (build-tree (- depth 1))
      (build-tree (- depth 1)))))
(build-tree 3)

(define a 1)

;;when = if begin
(when (> a 2)
  (display "hello"))

(use-modules (ice-9 match))

(map (match-lambda
       ((a . b) (display (string-append a " " b)))) (list '("111" . "222") '("333" . "444")))

(define (match-lambda
          ((a . b) (display (string-append a " " b)))))

