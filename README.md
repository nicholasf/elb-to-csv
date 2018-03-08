Transforms an access log for a classic ELB into a CSV.
# Install

Just git clone this repo. 

Any version of Node should work - the only dependencies are `fs.readdir` and `fs.readFile`.
# Running

Put your access logs into a `./logs` dir beneath the project root then use `npm start`. A `data.csv` will be produced.

# Customising

This was written to deal with certain conditions in access logs for the stack I work on. Just adapt index.js to the conditions you want to check for.
