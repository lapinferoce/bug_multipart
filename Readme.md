DON'T USE THIS 
--------------

this is a code sample to show how multipart(or hyper or rustfull) fails to upload big file 

build 
======

$cargo run


push a big file 
===============

in ./sample/huge
curl -v -F uploaded=@1BigFile.txt http://localhost:8080/content/test
