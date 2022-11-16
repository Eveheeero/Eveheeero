// https://docs.python.org/ko/3.9/c-api/concrete.html#fundamental-objects 에 각각의 형에 대해 정의되어있다.
// https://docs.python.org/ko/3.9/c-api/arg.html 로 인자를 파싱할 수 있다.
// https://numpy.org/doc/stable/reference/c-api/array.html?highlight=pyarray_fromany#data-access 로 numpy 데이터를 가져올 수 있다.
#define PY_SSIZE_T_CLEAN
#define NPY_NO_DEPRECATED_API NPY_1_7_API_VERSION
#include <Python.h>
#include <numpy/ndarrayobject.h>

static PyObject*
TestFunction(PyObject* self, PyObject* args)
{
  // Load Dataframe
  PyArrayObject* df;
  if (!PyArg_ParseTuple(args, "O", &df)) {
    return NULL;
  }
  printf("OK!\n");

  PyObject* data   = PyArray_GETITEM(df, PyArray_GETPTR1(df, 0));
  int       result = PyBool_Check(data);
  if (result == 1) {
    printf("True\n");
  }

  printf("OK!\n");

  if (data == Py_True) {
    printf("True\n");
  } else if (data == Py_False) {
    printf("False\n");
  } else {
    printf("Not a boolean\n");
  }

  Py_RETURN_NONE;
}

static PyMethodDef cnumpyMethod[] = {
  {"test", TestFunction, METH_VARARGS, "Test function"},
  {  NULL,         NULL,            0,            NULL}
};

static struct PyModuleDef cnumpy = {
  PyModuleDef_HEAD_INIT, // Initial
  "cnumpy",              // Module Name
  "c + numpy",           // Module Description
  -1,                    // Module Size
  cnumpyMethod           // Module Method
};

PyMODINIT_FUNC
PyInit_cnumpy(void)
{
  printf("MyModule Init\n");
  // Create Module
  PyObject* module = PyModule_Create(&cnumpy);

  return module;
}