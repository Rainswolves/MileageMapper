# Import Clap for CLI management
    - create manpage or some way to know how to use (the oso func)

# Make Linear or Oscolating options

The ocsolating part should take in a vector of numbers
The vector of numbers represents the amount of linear transactions

e.g.:

2 4 3 5

Each of the above represent the number of days in the week.
For each day in the week, the mileage is static, meaning
if inital amount is 100,000, then:

2 =>
    100,006
    100,012

# Since we are now done with the above 2, we call the add rnd() to add
# random amounts of mileage (in range) to the existing amount.
# This simulates vechicular activity between weekend and new week.

# In the below case, we added 8 as our rnd amount, so we start at: 100,020.
4 =>
    100,026
    100,032
    100,038
    100,044

# again, we add random amounts since we are done with the above 4, in this case add 6:
3 =>
    100,056
    100,062
    100,068   


#### Instead of linear function, look into linear closure!
