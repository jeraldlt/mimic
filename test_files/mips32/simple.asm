.data
  test1: .asciiz "Test String 1\n"
  test2: .asciiz "Test String 2\n"
  test3: .asciiz "Test String 3\n"

.text
main:
  j print_test3
print_test1:
  li $v0, 4
  la $a0, test1
  syscall
  j exit

print_test2:
  li $v0, 4
  la $a0, test2
  syscall
  j print_test1
  
print_test3:
  li $v0, 4
  la $a0, test3
  syscall
  j print_test2

exit:
  li $v0, 10
  syscall