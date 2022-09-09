#
# Print hello world and do a simple calculation
#

.0 "hello world" # Set memory location 0 to string "hello world"

$print .0 # Print memory location 0

_d .0 # Delete memory location 0

.0 1 # Set memory location 0 to integer 1
.1 2 # Set memory location 1 to integer 2
.2 0 # Set memory location 2 to integer 0 (To store the result of the calculation)

_a .0 .1 .2 # Add memory location 0 and 1 and store the result in memory location 2

$print .2 # Print memory location 2

_d .0 # Delete memory location 0
_d .1 # Delete memory location 1
_d .2 # Delete memory location 2
