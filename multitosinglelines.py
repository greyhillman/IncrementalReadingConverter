import argparse

def multi_to_single(lines):
    def helper(acc, x):
        if acc[-2:] == "- ":
            return acc[:-2] + x
        return acc + x + " "

    return reduce(helper, lines.split('\n'), "").strip()

def parseArgs():
    parser = argparse.ArgumentParser(
                description="Replaces multi-lines with a single line."
             )
    parser.add_argument('--debug', action="store_true")
    parser.add_argument('filename', help="The name of the file to convert")

    return parser.parse_args()

def convert_file(contents):
    def add(acc, x):
        if not x:
            return acc
        return acc + multi_to_single(x) + "\n\n"

    return reduce(add, contents.split('\n\n'), "").strip()

def main():
    args = parseArgs()
    debug = args.debug
    filename = args.filename

    with open(filename, "r") as file:
        contents = file.read()

    with open(filename + ".out", "w") as file:
        file.write(convert_file(contents))

if __name__ == "__main__":
    main()
