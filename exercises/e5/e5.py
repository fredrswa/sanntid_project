import os



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

    input("Press Enter to continue -- Message Passing")
    os.chdir('/home/fredrswa/Documents/00 Sanntid/git - project/Sanntid13/exercises/e5/messagepassing')
    os.system('go run priorityselect.go')    


main()