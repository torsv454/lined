# LinEd

A simple program that performs line editing, written as part of me trying to learn Rust.  

## Usage 

´´´
$ cat sample.txt
    Back,
    Forward,
    ForwardWord,
    BackWord,
    Home,
    End,

$ cat toconstants.txt
mark forward_word back_word cut
mark end delete cut
insert "const KW_" 
mark paste upcase 
insert ": &str = \"" 
mark paste downcase
insert "\";"

$ lined -f toconstants.txt < sample.txt
const KW_BACK,: &str = "back,";
const KW_FORWARD,: &str = "forward,";
const KW_FORWARDWORD,: &str = "forwardword,";
const KW_BACKWORD,: &str = "backword,";
const KW_HOME,: &str = "home,";
const KW_END,: &str = "end,";
$
´´´