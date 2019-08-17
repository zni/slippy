(define (map f x)
    (if (null? x)
        (quote ())
        (cons (f (car x)) (map f (cdr x)))))

(map (lambda (x) (+ x 4)) (quote (1 2 3 4)))
