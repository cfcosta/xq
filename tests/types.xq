# Pushing the wrong kind of data raises error
open a :integer
assert error (enqueue a 3.01)
