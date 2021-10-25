# Unitialized queues have length 0
assert (length a) 0
enqueue a 999
assert (length a) 1
assert (dequeue a) 999
assert (length a) 0
# We can enqueue multiple types
enqueue a "this"
assert (dequeue a) "this"
# Even on the same queue
enqueue a 1
enqueue a "foobar"
enqueue a 2
enqueue a 3
assert (length a) 4
dequeue a
# Peek gets the value in front of the queue without changing it
assert (peek a) "foobar"
dequeue a
dequeue a
assert (peek a) 3
assert (dequeue a) 3
