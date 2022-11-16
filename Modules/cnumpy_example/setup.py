from distutils.core import Extension, setup
import numpy as np

files = Extension('cnumpy', sources=['main.c'],
                  include_dirs=["/usr/lib/python3.9", np.get_include()])

setup(
    name="cnumpy",
    ext_modules=[files]
)
