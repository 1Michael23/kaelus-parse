# Todo

## Add parsing warnings to detect bad data.

User Error:
- too many tags on one cable
- cables with missing tags
- multiple cables with identical lengths
- multiple cables with identical VSWR
- multiple cables with identical return loss
- (search.rs) data missing from search

Bad Data: 
- Missing CSV viles
- Missing image thumbnails 
- 

## Search.rs

allow searching for cable tags in an external data format for a certain pattern such as a level in a tag. 

example:
(F-*L32*-432), regex match for the cables of the same floor that are missing.

Data Formats:
- XLSX 
- XLS 
- JSON
- CSV

Features 
- Regex cable search
- find any cables in dataset with no data yet.
- option to append extra cables to dataset
- 