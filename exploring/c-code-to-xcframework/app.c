#include <stdio.h>
#include "include/libmymath.h"
 
int main ( )
{
 
 double n = 5.0;

 printf ("%.2f to the power of 2 is %.2f \n", n, PowerOf2(n));
 printf ("%.2f to the power of 3 is %.2f \n", n, PowerOf3(n));

 return 0;
 
}
