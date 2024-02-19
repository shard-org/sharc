/THING = test

:name hello_world
:arch x86_64 linux

:verb run /bin/sh {
    sharc #FILE
    chmod +x #NAME
    ./#NAME
}

/HELLO = "hello, world!"

main entry:
    $puts #HELLO
    ret
