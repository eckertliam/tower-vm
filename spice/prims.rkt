#lang racket

(define (atom->string atom)
  (cond
    ((symbol? atom) (symbol->string atom))
    ((number? atom) (number->string atom))
    ((boolean? atom) (if (equal? #t atom)
                         "true"
                         "false"))
    (else atom)))

(define (binary-op op lhs rhs)
  (string-append (atom->string lhs)
                 " "
                 (atom->string op)
                 " "
                 (atom->string rhs)))

(define (new-const id value)
  (string-append "const "
                 (atom->string id)
                 " = "
                 (atom->string value)))

(define (new-var id value)
  (string-append "var "
                 (atom->string id)
                 " = "
                 (atom->string value)))

(define (new-args args)
  (define (aux args)
    (cond
      ((null? args) "")
      (else (string-append (atom->string (car args))
                           ", "
                           (aux (cdr args))))))
  (string-append "("
                 (aux args)
                 ")"))

(define (new-body body)
  (define (aux body)
    (cond
      ((null? body) "")
      (else (string-append (car body)
                           ";\n"
                           (aux (cdr body))))))
  (string-append "{\n"
                 (aux body)
                 "}"))

(define (new-if test body)
  (string-append "if ("
                 test
                 ")"
                 body))

