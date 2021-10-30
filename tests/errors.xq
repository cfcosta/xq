# Dequeuing from a closed queue raises error
assert error (dequeue a)
# Pushing the wrong kind of data raises error
open a :integer
assert error (enqueue a 3.01)
