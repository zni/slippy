;; lib.ss
;; library functions

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; numerical functions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(define (zero? n)
  (= n 0))

(define (positive? n)
  (> n 0))

(define (negative? n)
  (< n 0))

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;; list functions
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(define (caar list)
  (car (car list)))

(define (cadr list)
  (car (cdr list)))

(define (cdar list)
  (cdr (car list)))

(define (cddr list)
  (cdr (cdr list)))

(define (caaar list)
  (car (car (car list))))

(define (caadr list)
  (car (car (cdr list))))

(define (cadar list)
  (car (cdr (car list))))

(define (cdaar list)
  (cdr (car (car list))))

(define (cdadr list)
  (cdr (car (cdr list))))

(define (caddr list)
  (car (cdr (cdr list))))

(define (cddar list)
  (cdr (cdr (car list))))

(define (cdddr list)
  (cdr (cdr (cdr list))))

(define (caaaar list)
  (car (car (car (car list)))))

(define (cdaaar list)
  (cdr (car (car (car list)))))

(define (cadaar list)
  (car (cdr (car (car list)))))

(define (caadar list)
  (car (car (cdr (car list)))))

(define (caaadr list)
  (car (car (car (cdr list)))))

(define (caaddr list)
  (car (car (cdr (cdr list)))))

(define (cadddr list)
  (car (cdr (cdr (cdr list)))))

(define (cddddr list)
  (cdr (cdr (cdr (cdr list)))))

(define (cddaar list)
  (cdr (cdr (car (car list)))))

(define (cdddar list)
  (cdr (cdr (cdr (car list)))))

(define (caddar list)
  (car (cdr (cdr (car list)))))

(define (cdaadr list)
  (cdr (car (car (cdr list)))))

(define (cdadar list)
  (cdr (car (cdr (car list)))))

(define (cadadr list)
  (car (cdr (car (cdr list)))))

(define (cddadr list)
  (cdr (cdr (car (cdr list)))))

(define (cdaddr list)
  (cdr (car (cdr (cdr list)))))

(define (list-tail x k)
  (if (zero? k)
      x
      (list-tail (cdr x) (- k 1))))

(define (list-ref x k)
  (if (zero? k)
      (car x)
      (list-ref (cdr x) (- k 1))))
