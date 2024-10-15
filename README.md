 A library made for easier command line argument management

 Initially it was supposed to be a boilerplate module for my own command line utilities but it looked pretty easy to make a library
 You might want to write this yourself for your usecase if my library doesn't help you.
 
 At the moment I haven't tested this library fully, please beware of possible errors even though there shouldn't be any.
 
 uhm.. functions included in the library:
 -    ```get_full_args```  Returns ```env::args().collect()``` (the arguments)
 -    ```get_lones```      Returns the lone / independent arguments in order. Returns an empty vec if there is none
 -    ```get_next_arg```   Returns the specified argument / option’s next argument in an ```Option<String>```
 -    ```get_options```    Returns the options (“-argument”) in a vector. Returns an empty vector if there is none
 -    ```get_place```      Uses the input text and tells you where it is in the args vector, returns an ```Option<usize>```
 -    ```is_arg```         Checks if an argument exists