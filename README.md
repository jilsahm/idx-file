# IdxFile

A simple library for manipulating idx files written in Rust. 

## About the format

The idx file is written as binary with big endian byteorder. The header contains some meta data about the content. The following sample shows a simple idx layout containing one dimension with five elements each an unsigned byte.

```
        00  00  08  01  00  00  00  05  00  01  02  03  04
         |   |   |   |    \__|   |   |   |   |   |   |   |
   ZeroByte  |   |   |        \__|   |  Data ... ... ... ...
       ZeroByte  |   |            \__|
           DataType  |               |
              Dimensions     DimensionSize
```

The first four bytes are called the magic number. It contains two leading zerobytes. the third byte defines the datatype. The fourth byte describes how
many dimensions are used. The first dimension always describes how many vectors/matrices the file contains in total.

* One dimension: The file contains 1 row with n elements
* Two dimensions: The file contains n rows with m elements in each row
* Three or more dimensions: The file contains n rows with m * (m2 * ... * mx) elements in each row.

## Datatypes

The following datatypes are currently supported.

| ByteValue | DataType | Size |
| --- | --- | --- |
| 0x08 | unsigned byte | 8bit |
| 0x09 | signed byte | 8bit |
| 0x0B | signed short | 16bit |
| 0x0C | signed int | 32bit |
| 0x0D | float | 32bit |
| 0x0E | double | 64bit |
