
//#define NO_UNIX_SOCKETS
//#define NO_OPENSSL
//#define NO_NORETURN
//#define NO_MMAP
//#define NO_ST_BLOCKS_IN_STRUCT_STAT

#include "git-compat-util.h"
#include "strbuf.h"

#include "commit.h"

#define USE_THE_REPOSITORY_VARIABLE
#include "environment.h"

#include "tree.h"
#include "ident.h"
#include "repository.h"

//#include "builtin.h"
//#include "environment.h"
//#include "hex.h"
//#include "lockfile.h"
//#include "merge-ort.h"
//#include "object-name.h"
//#include "parse-options.h"
//#include "refs.h"
//#include "revision.h"
//#include "strmap.h"
//#include "oidset.h"
