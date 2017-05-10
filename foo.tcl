# location of the dll you compiled with Rust
load "..../foo.dll"

# creates two labels and a text entry widget, and packs them to the main window
# the text entered into the text entry will be stored in variable "friend"
pack [label .demo -text "Please enter something in the textbox below"] -pady 10 -padx 5
pack [entry .who -textvariable friend] -pady 10
pack [label .greeting] -pady 10

# when variable "friend" is written to, calls the greetFriend proc.
trace add variable friend write greetFriend

proc greetFriend args {
# makes global var friend visible
  global friend
# the following line calls my Rust command
  mycmd greet .greeting $friend
}
