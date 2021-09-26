assert (length a) 0
enqueue a 999
assert (length a) 1
assert (dequeue a) 999
assert (length a) 0
enqueue a "this"
assert (dequeue a) "this"
enqueue a 1
enqueue a "foobar"
enqueue a 2
enqueue a 3
assert (length a) 4
dequeue a
assert (peek a) "foobar"
dequeue a
dequeue a
assert (peek a) 3
assert (dequeue a) 3
