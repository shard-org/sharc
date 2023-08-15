/* 
 *  simple hello world program
 */

@main
   ;out: [a8] = "Hello, world!"

   ;i = 0
   @loop
      ;temp = [out + i]
      inc i
   (temp != 0) => #loop

   *stdout i, out // print the stuff

   // exit with status 0
   *ext 0
