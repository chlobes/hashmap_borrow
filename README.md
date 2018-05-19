adds a struct for safely borrowing multiple mutable references to a hashmap, I believe it is not UB because the mutable pointers do not overlap.
