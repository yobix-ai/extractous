import os
from platform import system

# On Windows there is no equivalent way of setting RPATH
# This adds the current directory to PATH so that the graalvm libs will be found 
if system() == "Windows":
    libpath = os.path.dirname(__file__)
    os.environ["PATH"] = libpath + os.pathsep + os.environ["PATH"]

from ._extractous import *

__doc__ = _extractous.__doc__
__all__ = _extractous.__all__