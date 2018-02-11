import os
import sys
import unittest

import hypothesis

from hypothesis import assume, given
from hypothesis import strategies as st

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), 'target/debug'))
import librust2py

class TestIt(unittest.TestCase):
    @given(st.lists(st.integers(max_value=2**64-1), min_size=10))
    def test_it(self, xs):
        assume(xs != sorted(xs))
        
        sorted_by_python = sorted(xs * 10)
        sorted_by_rust = librust2py.sort(xs * 10)
        self.assertEqual(sorted_by_python, sorted_by_rust)

if __name__ == '__main__':
    librust2py.version()
    unittest.main()