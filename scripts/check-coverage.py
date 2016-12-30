import json
import re
import sys

minimum_coverage = float(sys.argv[1])

# jsonfile is actually a js file with js objects
with open("target/cov/index.json") as jsonfile:
    data = jsonfile.readlines()

# here we will crudely turn the object into actual json
jsondata = ""
for line in data[:]:
    line = re.sub("var data = {files", "{\"files\"", line)
    line = re.sub("]};", "]}", line)
    line = re.sub("var .*", "", line)
    line = re.sub("\n", "", line)
    jsondata += line


jsondata = re.sub(",}", "}", jsondata)
jsondata = re.sub(",]", "]", jsondata)
jsondata = re.sub("merged_files", "\"merged_files\"", jsondata)

data = json.JSONDecoder().decode(jsondata)

coverage = float(data['merged_files'][0]['covered'])

if coverage < minimum_coverage:
    print "Unittest coverage below %0.2f." % minimum_coverage
    print "Coverage is currently:  %0.2f." % coverage
    print "Aborting commit..."
    sys.exit(1)
