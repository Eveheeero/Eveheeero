// With install python-dev package via apt or apk...
// All structures in this data changes with python version
// https://docs.python.org/3/extending/extending.html
// https://docs.python.org/ko/dev/c-api/typeobj.html
#define PY_SSIZE_T_CLEAN
#include <python3.9/Python.h>

struct MyClass
{
  PyObject_HEAD // Initialize
    int number; // Member
};

// static PyTypeObject MyClassData = {
//   PyVarObject_HEAD_INIT(0, 0) // Initialize
//   "MyModule.MyClass",         // tp_name
//   sizeof(struct MyClass),     // tp_basicsize
// };
static PyTypeObject MyClassData = {
  PyVarObject_HEAD_INIT(0, 0)             // Initial
    .tp_name    = "MyModule.MyClass",     // Name
  .tp_basicsize = sizeof(struct MyClass), // Size
};

static PyObject*
PrintHello(PyObject* self, PyObject* _NoArg)
{
  // Argument help.
  // https://docs.python.org/ko/3/c-api/arg.html
  printf("Hello World!\n");
  return Py_None;
}

// Define Module Methods
static PyMethodDef MyModuleMethod[] = {
  {"Helpme", (PyCFunction)PrintHello, METH_NOARGS, "Print Hello."}, // {Name, Function, ArgsRule, Doc}
  {    NULL,                    NULL,           0,           NULL}  // Essential (Maybe?)
};

// Define Module Information
static struct PyModuleDef MyModule = {
  PyModuleDef_HEAD_INIT,  // Initial
  "MyModule",             // Module Name
  "MyModule Description", // Module Description
  -1,                     // Module Size
  MyModuleMethod          // Module Method
};

PyMODINIT_FUNC
PyInit_MyModule(void)
{
  printf("MyModule Init\n");
  // Create Module
  PyObject* module = PyModule_Create(&MyModule);

  // Define Custom Class
  PyType_Ready(&MyClassData);
  Py_IncRef((PyObject*)&MyClassData); // Add Reference to Class (avoid GC)
  PyModule_AddObject(module, "MyClass", (PyObject*)&MyClassData);

  return module;
}
