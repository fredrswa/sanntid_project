import os
import sys

def tester(result):
    test1 = result[0:3]
    test2 = result[3:12]
    test3 = result[12:]
    print("Test 1: ", test1)
    print("Test 2: ", test2)
    print("Test 3: ", test3)
    all_tests = True
    for i in range(len(test1)):
        if i != test1[i]:
            print("Test 1 failed")
            all_tests = False
            break
    for i in test2[0:5]:
        if i % 2 != 0:
            print("Test 2 failed")
            all_tests = False
            break
    
    if [7, 6] in test3[-2:] or [6, 7] in test3[-2:]:
        print("Test 3 failed")
        all_tests = False

    if all_tests:
        print("All tests passed")
    else:
        print("Some tests failed")
    

def main():
    input("Press Enter to continue -- Semaphores")
    os.chdir('/home/fredrswa/Documents/00 Sanntid/git - project/Sanntid13/exercises/e5/semaphore')
    os.system('dmd -run semaphore.d')

    input("Press Enter to continue -- Condition Variables")
    os.chdir('/home/fredrswa/Documents/00 Sanntid/git - project/Sanntid13/exercises/e5/conditionvariable')
    os.system('dmd -run condvar.d')

    input("Press Enter to continue -- Protected Objects")
    os.chdir('/home/fredrswa/Documents/00 Sanntid/git - project/Sanntid13/exercises/e5/protectedobject')
    os.system('./protectobj')
    # Read the output of ./protectobj
    result = os.popen('./protectobj').read().strip().split('\n')[-1].split(' ')[1:]
    result = [int(i) for i in result]
    tester(result)


        

    input("Press Enter to continue -- Message Passing")
    os.chdir('/home/fredrswa/Documents/00 Sanntid/git - project/Sanntid13/exercises/e5/messagepassing')
    os.system('go run priorityselect.go')   
    os.system('go run request.go') 


main()