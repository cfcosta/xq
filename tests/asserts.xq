enqueue a 1
enqueue a 2
assert (peek a) 1
assert (length a) 2
dequeue a
assert (peek a) 2
assert (length a) 1
enqueue a 1
enqueue a 2
enqueue a 1
enqueue a 2
enqueue a 1
enqueue a 2
enqueue a 1
enqueue a 2
assert (length a) 9
dequeue a
dequeue a
dequeue a
dequeue a
dequeue a
dequeue a
assert (length a) 3
