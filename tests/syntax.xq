# Closed queues can not be modified or queried in any way
assert error (enqueue a 1)
assert error (dequeue a)
assert error (peek a)
assert error (length a)
# We can open keys with different data types
open q_int :integer
open q_float :float
open q_string :string
# All open queues start with length 0
assert (length q_int) 0
assert (length q_float) 0
assert (length q_string) 0
# Peeking at an empty queue always returns null
assert (peek q_int) null
assert (peek q_float) null
assert (peek q_string) null
# Peeking at an empty queue always returns null
assert (dequeue q_int) null
assert (dequeue q_float) null
assert (dequeue q_string) null
# We can enqueue data to a queue, as long as the types match
enqueue q_int 1
assert error (enqueue q_int 1.01)
assert error (enqueue q_int "foo")
enqueue q_float 1.01
assert error (enqueue q_float 1)
assert error (enqueue q_float "foo")
enqueue q_string "foo"
assert error (enqueue q_string 1)
assert error (enqueue q_string 1.01)
# We can get the length of any queue
assert (length q_int) 1
enqueue q_int 3
assert (length q_int) 2
assert (length q_float) 1
enqueue q_float 2.01
assert (length q_float) 2
assert (length q_string) 1
enqueue q_string "omg"
assert (length q_string) 1
# We can also peek the head of the queue
assert (peek q_int) 1
assert (peek q_float) 1.01
assert (peek q_string) "foo"
# Then we can close the queues
close q_int :integer
close q_float :float
close q_string :string
# And running operations on closed queues errors out, even if it was available
# at some point.
assert error (enqueue q_int 1)
assert error (dequeue q_int)
assert error (peek q_int)
assert error (length q_int)
