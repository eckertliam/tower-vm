#lang info
(define collection "spice")
(define deps '("base"))
(define build-deps '("scribble-lib" "racket-doc" "rackunit-lib"))
(define scribblings '(("scribblings/spice.scrbl" ())))
(define pkg-desc "A scheme dialect for the future of the web.")
(define version "0.1")
(define pkg-authors '(liameckert))
(define license '(MIT))
