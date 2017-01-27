import re
import argparse

from flatten import flatten

def parseArgs():
    parser = argparse.ArgumentParser(description="Convert HTML files into Anki notes.")
    parser.add_argument('--debug', action="store_true")
    parser.add_argument('filename', help='The name of the file to be converted') 

    return parser.parse_args()

def main():
    args = parseArgs()
    filename = args.filename
    debug = args.debug

    with open(filename, "r") as file:
        contents = file.read().decode('utf8')

    contents = re.sub("<br>", "<br />", contents)

    contents = flatten(contents)

    with open(filename + ".anki", "w") as file:
        if not debug:
            contents = "<br />".join(contents.split('\n'))
        file.write(contents.encode('utf8'))

if __name__ == "__main__":
    main()
