(define stack '())

(define (pop)
  (if (null? stack)
      'error
      (let ((a (car stack)))
        (set! stack (cdr stack))
        a)))

(define (push n)
  (set! stack (cons n stack)))
