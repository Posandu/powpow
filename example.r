# 
# Basic stuff
#
.0 "Hello World" # Initialize memory address 0 with the string "Hello World"
$print .0 # Execute the print function with the argument .0
_d .0 # Delete memory address 0

# Add two numbers
.0 1 # Initialize memory address 0 with the number 1
.1 2 # Initialize memory address 1 with the number 2
.2 0 # Memory address 2 is used as a temporary variable to store the result
_a .0 .1 .2 # Execute the add function with the arguments .0, .1 and store the result in .2
$print .2 # Print the result

_d .0 # Delete memory address 0
_d .1 # Delete memory address 1
_d .2 # Delete memory address 2

