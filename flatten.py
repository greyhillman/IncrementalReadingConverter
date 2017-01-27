import re
import argparse

from bs4 import BeautifulSoup # html parser

def parseArgs():
    parser = argparse.ArgumentParser(description="Convert HTML files into Anki notes.")
    parser.add_argument('--debug', action="store_true")
    parser.add_argument('filename', help='The name of the file to be converted') 

    return parser.parse_args()

def strip(string):
    result = string.strip()
    if string[0] == ' ':
        result = " " + result
    if string[-1] == ' ':
        result = result + " "
    return result

def process_img(node):
    def defolder(string):
        return string.split('/')[-1]
    return "<img src=\"" + defolder(node['src']) + "\" />"

def process_p(node):
    return default_process(node) + "\n\n"

def process_nothing(node):
    return ""

def process_div(node):
    return default_process(node).strip() + "\n\n"

def process_pre(node):
    def helper(node):
        if not node.name:
            return node.string

        def help(acc, x):
            return acc + process(x)
        return reduce(help, node.children, "")
    return helper(node)

def process_li(node):
    return default_process(node).strip() + "\n"

def process_ol(node):
    content = default_process(node)
    def helper(acc, x):
        x = x.strip()
        if not x:
            return acc
        i = len(acc.split('\n')) - 1
        return acc + str(i) + ")" + x + "\n"
    return reduce(helper, content.split('\n'), "\n")

def process_ul(node):
    content = default_process(node)
    def helper(acc, x):
        if not x:
            return acc
        return acc + "--" + x + "\n"
    return reduce(helper, content.split('\n'), "\n")

def process_br(node):
    # BS misreads this type of tag and thinks it holds the
    # rest of the document. The tag should be <br />, but it's
    # malformed
    return "\n" + default_process(node)

PROCESS_LIST = [
    (['p', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6'], process_p),
    (['head', 'script', 'style'], process_nothing),
    (['img'], process_img),
    (['div'], process_div),
    (['li'], process_li),
    (['ol'], process_ol),
    (['ul'], process_ul),
    (['br'], process_br),
    (['pre'], process_pre),
]

def default_process(node):
    # This node only contains text with nothing else in it
    if not node.name:
        return strip(node.string)

    def helper(acc, x):
        return acc + process(x)
    return reduce(helper, node.children, "")

def process(node):
    def get_function():
        def helper(x):
            return node.name in x[0]
        result = filter(helper, PROCESS_LIST)
        if result:
            return result[0][1] # first element's function
        return None
    process_f = get_function()

    if process_f:
        return process_f(node)
    else:
        return default_process(node)

def flatten(contents):
    soup = BeautifulSoup(contents, 'html.parser')
    return process(soup.body)
