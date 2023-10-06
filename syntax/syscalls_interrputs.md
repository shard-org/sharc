things prefixed with `opt_` are optional

# fs
## file descriptors
```
STDIN   0    // standard input
STDOUT  1    // standard output
STDERR  2    // standard error

// errors (negative numbers)
EPERM   -1   // Operation not permitted
ENOENT  -2   // No such file or directory
ESRCH   -3   // No such process
EINTR   -4   // Interrupted system call
EIO     -5   // I/O error
ENXIO   -6   // No such device or address
E2BIG   -7   // Argument list too long
ENOEXEC -8   // Exec format error
EBADF   -9   // Bad file number
ECHILD  -10  // No child processes
EAGAIN  -11  // Try again
ENOMEM  -12  // Out of memory
EACCES  -13  // Permission denied
EFAULT  -14  // Bad address
ENOTBLK -15  // Block device required
EBUSY   -16  // Device or resource busy
EEXIST  -17  // File exists
EXDEV   -18  // Cross-device link
ENODEV  -19  // No such device
ENOTDIR -20  // Not a directory
EISDIR  -21  // Is a directory
EINVAL  -22  // Invalid argument
ENFILE  -23  // File table overflow
EMFILE  -24  // Too many open files
ENOTTY  -25  // Not a typewriter
ETXTBSY -26  // Text file busy
EFBIG   -27  // File too large
ENOSPC  -28  // No space left on device
ESPIPE  -29  // Illegal seek
EROFS   -30  // Read-only file system
EMLINK  -31  // Too many links
EPIPE   -32  // Broken pipe
EDOM    -33  // Math argument out of domain of func
ERANGE  -34  // Math result not representable
```


## read 0
reads the contents of a file given the file descriptor
```
*read fd, buffer_ptr, amount
```

example:
```
*open "file.txt", 0, 0
%buffer = $malloc 1024
*read some_file, [buffer], 1024
```

## write 1
writes to a file given the file descriptor
```
*write fd, buffer_ptr, amount
```

example:
```
*write STDOUT, "Hello World!", 12
```

shorthands:
```
*stdout "hello, world!\n", 14
*stderr "hello, world!\n", 14
```


## open 2
opens the given file, returning a file descriptor
```
*open path, flags, opt_mode  // returns file descriptor (fd)
```

example:
```
;file = *open "file.txt", 0, 0
```

flags:
```
CREAT        1000   // create if it doesn't exist
TRUNC        2000   // truncate file
EXCL         4000   // fail if exists
NOCTTY      10000  // don't make it a controlling terminal

NONBLOCK        4  // non-blocking I/O
APPEND         10  // append (writes guaranteed at the end)
DSYNC       40000  // sync data (but not metadata unless FSYNC)
DIRECTORY  100000  // must be a directory
NOFOLLOW   200000  // don't follow links
LARGEFILE  400000  // allow large files (LFS)
DIRECT    2000000  // direct disk access 
NOATIME   4000000  // don't update atime
CLOEXEC  10000000  // set close_on_exec
```

mode:
```
RDONLY 0  // read
WRONLY 1  // write
RDWR   2  // read write
```

## close 3
closes an opened file descriptor
```
*close fd
```

example:
```
;file = *open "file.txt", 0, 0
*close file
```


## socket 41
creates a socket
```
*socket domain, type, opt_protocol
```

example:
```
// this creates a tcp socket
;sock = *socket AF_INET, SOCK_STREAM  // returns socket fd
```

domains:
```
AF_UNIX       1   // unix domain sockets
AF_INET       2   // IPv4 internet protocols
AF_AX25       3   // Amateur Radio AX.25
AF_IPX        4   // Novell Internet Protocol
AF_APPLETALK  5   // AppleTalk
AF_NETROM     6   // Amateur Radio NET/ROM
AF_BRIDGE     7   // Multiprotocol bridge 
AF_ATMPVC     8   // ATM PVCs 
AF_X25        9   // ITU-T X.25 / ISO-8208 protocol
AF_INET6      10  // IPv6 internet protocols
AF_ROSE       11  // Amateur Radio X.25 PLP
AF_DECnet     12  // Reserved for DECnet project
AF_NETBEUI    13  // Reserved for 802.2LLC project
AF_SECURITY   14  // Security callback pseudo AF
AF_KEY        15  // PF_KEY key management API
AF_NETLINK    16  // NETLINK sockets
AF_PACKET     17  // Packet family
AF_ASH        18  // ASH
AF_ECONET     19  // Acorn Econet
AF_ATMSVC     20  // ATM SVCs 
AF_RDS        21  // RDS sockets 
AF_SNA        22  // Linux SNA Project 
AF_IRDA       23  // IRDA sockets 
AF_PPPOX      24  // PPPoX sockets 
AF_WANPIPE    25  // Wanpipe API sockets 
AF_LLC        26  // Linux LLC 
AF_IB         27  // Native InfiniBand address 
AF_MPLS       28  // MPLS
AF_CAN        29  // Controller Area Network 
AF_TIPC       30  // TIPC sockets
AF_BLUETOOTH  31  // Bluetooth sockets
AF_IUCV       32  // IUCV sockets
AF_RXRPC      33  // RxRPC sockets
AF_ISDN       34  // mISDN sockets
AF_PHONET     35  // Phonet sockets
AF_IEEE802154 36  // IEEE 802.15.4 sockets
AF_CAIF       37  // CAIF sockets
AF_ALG        38  // Algorithm sockets
AF_NFC        39  // NFC sockets
AF_VSOCK      40  // vSockets
AF_KCM        41  // Kernel Connection Multiplexor
AF_QIPCRTR    42  // Qualcomm IPC Router
AF_SMC        43  // smc sockets
AF_XDP        44  // XDP sockets
AF_MCTP       45  // managment component transport protocol 
AF_MAX        46  // For now.. 
```

types:
```
SOCK_STREAM    1  // sequenced, reliable, two-way, connection-based byte streams
SOCK_DGRAM     2  // connectionless, unreliable messages of a fixed maximum length
SOCK_RAW       3  // raw network protocol access
SOCK_RDM       4  // reliably-delivered messages
SOCK_SEQPACKET 5  // sequenced, reliable, two-way connection-based transmission, of fixed maximum length
SOCK_DCCP      6  // Datagram Congestion Control Protocol 
SOCK_PACKET    10 // obsolete
```

protocol should be 0 for most cases, if there's several possible protocols check linux syscalls manual

## bind 49
binds a socket to an address
```
*bind fd, SockAddr_ptr
```

example:
```
;sock = *socket AF_INET, SOCK_STREAM
;addr SockAddrIn = { AF_INET, 0x901f, INADDR_ANY, 0* }
*bind sock, addr
```

sockaddr:
```
.str SockAddr {
    2  family  // address family: AF_INET
    2  port    // port (big endian)
    4  addr    // internet address
    :8 zero    // padding
}
```

## listen 50
listens for connections on a socket
```
*listen fd, backlog
```

backlog is the maximum length of the queue of pending connections

example:
```
;sock = *socket AF_INET, SOCK_STREAM
;addr SockAddrIn = { AF_INET, 0x901f, INADDR_ANY, 0* }
*bind sock, addr 
*listen sock, 5
```


## execve 59
executes a program
```
*execve ptr_path, ptr_args, ptr_envp

// path: [string]
// args: {[string], [string], 0}
// envp: {[string], [string], 0}
```

example:
```
*execve "/bin/ls", {"ls", "-lh", 0}, 0
```

## exit 60
exits the program
```
*exit status
```

example:
```
*exit 0
```
