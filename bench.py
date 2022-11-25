import os
import sys
import time

def main():
    start_time = time.time()
    ind = 0
    counter = 0
    with open('./file.bin', 'rb') as f:
        while True:
            chunk = f.read(65536)
            if not chunk:
                break
            if b'--boundary--' in chunk:
                ind += chunk.index(b"--boundary--")
                counter += 1
                print("Number of chunks read: ", counter)
                print("String found at index: ", ind)
                break
            else :
                counter += 1
                ind += 65536
    end_time = time.time()
    time_lapsed = end_time - start_time
    print(time_lapsed)


if __name__ == '__main__':
    main()