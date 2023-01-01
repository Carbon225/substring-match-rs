# Substring Match

A Python library written in Rust that can be used to search a collection of strings for strings containing a pattern.

## Installation

```
pip install substring-match
```

## Usage

```python
from substring_match import SubstringMatcher
m = SubstringMatcher(["banana", "apple tree", "pineapple tree"])
m.find("apple")
# ["apple tree", "pineapple tree"]
```
