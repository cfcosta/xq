# Tests to help us support nulls on the correct cases
assert (dequeue a) null
assert (peek a) null
# Then we enqueue and dequeue again to make sure we're fine
enqueue a 1
assert (dequeue a) 1
