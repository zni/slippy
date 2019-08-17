(define stack '())

(define (run)
  (let ((n (read)))
    (cond
      ((number? n) (begin (push n) (run)))
      ((equal? n '+) (begin (add) (run)))
      ((equal? n '-) (begin (sub) (run)))
      ((equal? n '*) (begin (mul) (run)))
      ((equal? n '.s) (begin (display stack) (run)))
      ((equal? n '.exit) #f)
      (else (run)))))

(define (add)
  (let ((a (pop))
        (b (pop)))
    (let ((sum (+ a b)))
      (push sum)
      (display sum))))

(define (sub)
  (let ((a (pop))
        (b (pop)))
    (let ((n (- a b)))
      (push n)
      (display n))))

(define (mul)
  (let ((a (pop))
        (b (pop)))
    (let ((n (* a b)))
      (push n)
      (display n))))

(define (pop)
  (if (null? stack)
      'stack-underflow
      (let ((a (car stack)))
        (set! stack (cdr stack))
        a)))

(define (push n)
  (set! stack (cons n stack)))
