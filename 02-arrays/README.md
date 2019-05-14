# HW02 otus-algorithms

Homework contains implementations of: Array, SingleArray, VectorArray, FactorArray, MatrixArray, PriorityQueue

## Description

Add(idx, item) method implemented as insert instance method.
Remove(idx) method implemented through delitem and remove methods.
Add to the end of array through append method.

### Doc

Graphs and tables:
* [Append method comparsion](https://github.com/nekogda/otus-algorithms/blob/master/02-arrays/doc/arrays_append.png)
* [Insert method comparsion](https://github.com/nekogda/otus-algorithms/blob/master/02-arrays/doc/arrays_insert.png)

SingleArray не использовался при построении графиков, по причине низкой производительности.
Array был реализован поверх ctypes.py_object, т.к. по заданию запрещалось использовать встроенный array.

### Conclusion
* Существует множество возможностей реализации одного и того же абстрактного типа данных.
Если заренее известны основные профили использования, то можно выбрать реализацию, которая наиболее оптимально будет справляться с предложенными задачами.
* Наиболее эффективными оптимизациями для операций вставки оказались MatrixArray и FactorArray. Основное время в других реализациях затрачивалось на копирование элементов при реаллокациях.
* Python бесполезен для реализации алгоритмов критичных к времени выполнения