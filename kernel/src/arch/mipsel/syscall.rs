//! MIPS N32 ABI syscall ids
//! Reference: https://git.linux-mips.org/cgit/ralf/linux.git/tree/arch/mips/include/uapi/asm/unistd.h

extern crate paste;

pub const MIPS_SYSCALL_OFFSET: usize = 4000;

macro_rules! define_syscall {
    ($name: ident, $id: expr) => {
        paste::item! {
            pub const [<SYS_ $name>] : usize = MIPS_SYSCALL_OFFSET + $id;
        }
    };
}

define_syscall!(SYSCALL, 0);
define_syscall!(EXIT, 1);
define_syscall!(FORK, 2);
define_syscall!(READ, 3);
define_syscall!(WRITE, 4);
define_syscall!(OPEN, 5);
define_syscall!(CLOSE, 6);
define_syscall!(WAITPID, 7);
define_syscall!(CREAT, 8);
define_syscall!(LINK, 9);
define_syscall!(UNLINK, 10);
define_syscall!(EXECVE, 11);
define_syscall!(CHDIR, 12);
define_syscall!(TIME, 13);
define_syscall!(MKNOD, 14);
define_syscall!(CHMOD, 15);
define_syscall!(LCHOWN, 16);
define_syscall!(BREAK, 17);
define_syscall!(UNUSED18, 18);
define_syscall!(LSEEK, 19);
define_syscall!(GETPID, 20);
define_syscall!(MOUNT, 21);
define_syscall!(UMOUNT, 22);
define_syscall!(SETUID, 23);
define_syscall!(GETUID, 24);
define_syscall!(STIME, 25);
define_syscall!(PTRACE, 26);
define_syscall!(ALARM, 27);
define_syscall!(UNUSED28, 28);
define_syscall!(PAUSE, 29);
define_syscall!(UTIME, 30);
define_syscall!(STTY, 31);
define_syscall!(GTTY, 32);
define_syscall!(ACCESS, 33);
define_syscall!(NICE, 34);
define_syscall!(FTIME, 35);
define_syscall!(SYNC, 36);
define_syscall!(KILL, 37);
define_syscall!(RENAME, 38);
define_syscall!(MKDIR, 39);
define_syscall!(RMDIR, 40);
define_syscall!(DUP, 41);
define_syscall!(PIPE, 42);
define_syscall!(TIMES, 43);
define_syscall!(PROF, 44);
define_syscall!(BRK, 45);
define_syscall!(SETGID, 46);
define_syscall!(GETGID, 47);
define_syscall!(SIGNAL, 48);
define_syscall!(GETEUID, 49);
define_syscall!(GETEGID, 50);
define_syscall!(ACCT, 51);
define_syscall!(UMOUNT2, 52);
define_syscall!(LOCK, 53);
define_syscall!(IOCTL, 54);
define_syscall!(FCNTL, 55);
define_syscall!(MPX, 56);
define_syscall!(SETPGID, 57);
define_syscall!(ULIMIT, 58);
define_syscall!(UNUSED59, 59);
define_syscall!(UMASK, 60);
define_syscall!(CHROOT, 61);
define_syscall!(USTAT, 62);
define_syscall!(DUP2, 63);
define_syscall!(GETPPID, 64);
define_syscall!(GETPGRP, 65);
define_syscall!(SETSID, 66);
define_syscall!(SIGACTION, 67);
define_syscall!(SGETMASK, 68);
define_syscall!(SSETMASK, 69);
define_syscall!(SETREUID, 70);
define_syscall!(SETREGID, 71);
define_syscall!(SIGSUSPEND, 72);
define_syscall!(SIGPENDING, 73);
define_syscall!(SETHOSTNAME, 74);
define_syscall!(SETRLIMIT, 75);
define_syscall!(GETRLIMIT, 76);
define_syscall!(GETRUSAGE, 77);
define_syscall!(GETTIMEOFDAY, 78);
define_syscall!(SETTIMEOFDAY, 79);
define_syscall!(GETGROUPS, 80);
define_syscall!(SETGROUPS, 81);
define_syscall!(RESERVED82, 82);
define_syscall!(SYMLINK, 83);
define_syscall!(UNUSED84, 84);
define_syscall!(READLINK, 85);
define_syscall!(USELIB, 86);
define_syscall!(SWAPON, 87);
define_syscall!(REBOOT, 88);
define_syscall!(READDIR, 89);
define_syscall!(MMAP, 90);
define_syscall!(MUNMAP, 91);
define_syscall!(TRUNCATE, 92);
define_syscall!(FTRUNCATE, 93);
define_syscall!(FCHMOD, 94);
define_syscall!(FCHOWN, 95);
define_syscall!(GETPRIORITY, 96);
define_syscall!(SETPRIORITY, 97);
define_syscall!(PROFIL, 98);
define_syscall!(STATFS, 99);
define_syscall!(FSTATFS, 100);
define_syscall!(IOPERM, 101);
define_syscall!(SOCKETCALL, 102);
define_syscall!(SYSLOG, 103);
define_syscall!(SETITIMER, 104);
define_syscall!(GETITIMER, 105);
define_syscall!(STAT, 106);
define_syscall!(LSTAT, 107);
define_syscall!(FSTAT, 108);
define_syscall!(UNUSED109, 109);
define_syscall!(IOPL, 110);
define_syscall!(VHANGUP, 111);
define_syscall!(IDLE, 112);
define_syscall!(VM86, 113);
define_syscall!(WAIT4, 114);
define_syscall!(SWAPOFF, 115);
define_syscall!(SYSINFO, 116);
define_syscall!(IPC, 117);
define_syscall!(FSYNC, 118);
define_syscall!(SIGRETURN, 119);
define_syscall!(CLONE, 120);
define_syscall!(SETDOMAINNAME, 121);
define_syscall!(UNAME, 122);
define_syscall!(MODIFY_LDT, 123);
define_syscall!(ADJTIMEX, 124);
define_syscall!(MPROTECT, 125);
define_syscall!(SIGPROCMASK, 126);
define_syscall!(CREATE_MODULE, 127);
define_syscall!(INIT_MODULE, 128);
define_syscall!(DELETE_MODULE, 129);
define_syscall!(GET_KERNEL_SYMS, 130);
define_syscall!(QUOTACTL, 131);
define_syscall!(GETPGID, 132);
define_syscall!(FCHDIR, 133);
define_syscall!(BDFLUSH, 134);
define_syscall!(SYSFS, 135);
define_syscall!(PERSONALITY, 136);
define_syscall!(AFS_SYSCALL, 137);
define_syscall!(SETFSUID, 138);
define_syscall!(SETFSGID, 139);
define_syscall!(_LLSEEK, 140);
define_syscall!(GETDENTS, 141);
define_syscall!(_NEWSELECT, 142);
define_syscall!(FLOCK, 143);
define_syscall!(MSYNC, 144);
define_syscall!(READV, 145);
define_syscall!(WRITEV, 146);
define_syscall!(CACHEFLUSH, 147);
define_syscall!(CACHECTL, 148);
define_syscall!(SYSMIPS, 149);
define_syscall!(UNUSED150, 150);
define_syscall!(GETSID, 151);
define_syscall!(FDATASYNC, 152);
define_syscall!(_SYSCTL, 153);
define_syscall!(MLOCK, 154);
define_syscall!(MUNLOCK, 155);
define_syscall!(MLOCKALL, 156);
define_syscall!(MUNLOCKALL, 157);
define_syscall!(SCHED_SETPARAM, 158);
define_syscall!(SCHED_GETPARAM, 159);
define_syscall!(SCHED_SETSCHEDULER, 160);
define_syscall!(SCHED_GETSCHEDULER, 161);
define_syscall!(SCHED_YIELD, 162);
define_syscall!(SCHED_GET_PRIORITY_MAX, 163);
define_syscall!(SCHED_GET_PRIORITY_MIN, 164);
define_syscall!(SCHED_RR_GET_INTERVAL, 165);
define_syscall!(NANOSLEEP, 166);
define_syscall!(MREMAP, 167);
define_syscall!(ACCEPT, 168);
define_syscall!(BIND, 169);
define_syscall!(CONNECT, 170);
define_syscall!(GETPEERNAME, 171);
define_syscall!(GETSOCKNAME, 172);
define_syscall!(GETSOCKOPT, 173);
define_syscall!(LISTEN, 174);
define_syscall!(RECV, 175);
define_syscall!(RECVFROM, 176);
define_syscall!(RECVMSG, 177);
define_syscall!(SEND, 178);
define_syscall!(SENDMSG, 179);
define_syscall!(SENDTO, 180);
define_syscall!(SETSOCKOPT, 181);
define_syscall!(SHUTDOWN, 182);
define_syscall!(SOCKET, 183);
define_syscall!(SOCKETPAIR, 184);
define_syscall!(SETRESUID, 185);
define_syscall!(GETRESUID, 186);
define_syscall!(QUERY_MODULE, 187);
define_syscall!(POLL, 188);
define_syscall!(NFSSERVCTL, 189);
define_syscall!(SETRESGID, 190);
define_syscall!(GETRESGID, 191);
define_syscall!(PRCTL, 192);
define_syscall!(RT_SIGRETURN, 193);
define_syscall!(RT_SIGACTION, 194);
define_syscall!(RT_SIGPROCMASK, 195);
define_syscall!(RT_SIGPENDING, 196);
define_syscall!(RT_SIGTIMEDWAIT, 197);
define_syscall!(RT_SIGQUEUEINFO, 198);
define_syscall!(RT_SIGSUSPEND, 199);
define_syscall!(PREAD64, 200);
define_syscall!(PWRITE64, 201);
define_syscall!(CHOWN, 202);
define_syscall!(GETCWD, 203);
define_syscall!(CAPGET, 204);
define_syscall!(CAPSET, 205);
define_syscall!(SIGALTSTACK, 206);
define_syscall!(SENDFILE, 207);
define_syscall!(GETPMSG, 208);
define_syscall!(PUTPMSG, 209);
define_syscall!(MMAP2, 210);
define_syscall!(TRUNCATE64, 211);
define_syscall!(FTRUNCATE64, 212);
define_syscall!(STAT64, 213);
define_syscall!(LSTAT64, 214);
define_syscall!(FSTAT64, 215);
define_syscall!(PIVOT_ROOT, 216);
define_syscall!(MINCORE, 217);
define_syscall!(MADVISE, 218);
define_syscall!(GETDENTS64, 219);
define_syscall!(FCNTL64, 220);
define_syscall!(RESERVED221, 221);
define_syscall!(GETTID, 222);
define_syscall!(READAHEAD, 223);
define_syscall!(SETXATTR, 224);
define_syscall!(LSETXATTR, 225);
define_syscall!(FSETXATTR, 226);
define_syscall!(GETXATTR, 227);
define_syscall!(LGETXATTR, 228);
define_syscall!(FGETXATTR, 229);
define_syscall!(LISTXATTR, 230);
define_syscall!(LLISTXATTR, 231);
define_syscall!(FLISTXATTR, 232);
define_syscall!(REMOVEXATTR, 233);
define_syscall!(LREMOVEXATTR, 234);
define_syscall!(FREMOVEXATTR, 235);
define_syscall!(TKILL, 236);
define_syscall!(SENDFILE64, 237);
define_syscall!(FUTEX, 238);
define_syscall!(SCHED_SETAFFINITY, 239);
define_syscall!(SCHED_GETAFFINITY, 240);
define_syscall!(IO_SETUP, 241);
define_syscall!(IO_DESTROY, 242);
define_syscall!(IO_GETEVENTS, 243);
define_syscall!(IO_SUBMIT, 244);
define_syscall!(IO_CANCEL, 245);
define_syscall!(EXIT_GROUP, 246);
define_syscall!(LOOKUP_DCOOKIE, 247);
define_syscall!(EPOLL_CREATE, 248);
define_syscall!(EPOLL_CTL, 249);
define_syscall!(EPOLL_WAIT, 250);
define_syscall!(REMAP_FILE_PAGES, 251);
define_syscall!(SET_TID_ADDRESS, 252);
define_syscall!(RESTART_SYSCALL, 253);
define_syscall!(FADVISE64, 254);
define_syscall!(STATFS64, 255);
define_syscall!(FSTATFS64, 256);
define_syscall!(TIMER_CREATE, 257);
define_syscall!(TIMER_SETTIME, 258);
define_syscall!(TIMER_GETTIME, 259);
define_syscall!(TIMER_GETOVERRUN, 260);
define_syscall!(TIMER_DELETE, 261);
define_syscall!(CLOCK_SETTIME, 262);
define_syscall!(CLOCK_GETTIME, 263);
define_syscall!(CLOCK_GETRES, 264);
define_syscall!(CLOCK_NANOSLEEP, 265);
define_syscall!(TGKILL, 266);
define_syscall!(UTIMES, 267);
define_syscall!(MBIND, 268);
define_syscall!(GET_MEMPOLICY, 269);
define_syscall!(SET_MEMPOLICY, 270);
define_syscall!(MQ_OPEN, 271);
define_syscall!(MQ_UNLINK, 272);
define_syscall!(MQ_TIMEDSEND, 273);
define_syscall!(MQ_TIMEDRECEIVE, 274);
define_syscall!(MQ_NOTIFY, 275);
define_syscall!(MQ_GETSETATTR, 276);
define_syscall!(VSERVER, 277);
define_syscall!(WAITID, 278);
define_syscall!(SYS_SETALTROOT, 279);
define_syscall!(ADD_KEY, 280);
define_syscall!(REQUEST_KEY, 281);
define_syscall!(KEYCTL, 282);
define_syscall!(SET_THREAD_AREA, 283);
define_syscall!(INOTIFY_INIT, 284);
define_syscall!(INOTIFY_ADD_WATCH, 285);
define_syscall!(INOTIFY_RM_WATCH, 286);
define_syscall!(MIGRATE_PAGES, 287);
define_syscall!(OPENAT, 288);
define_syscall!(MKDIRAT, 289);
define_syscall!(MKNODAT, 290);
define_syscall!(FCHOWNAT, 291);
define_syscall!(FUTIMESAT, 292);
define_syscall!(FSTATAT64, 293);
define_syscall!(UNLINKAT, 294);
define_syscall!(RENAMEAT, 295);
define_syscall!(LINKAT, 296);
define_syscall!(SYMLINKAT, 297);
define_syscall!(READLINKAT, 298);
define_syscall!(FCHMODAT, 299);
define_syscall!(FACCESSAT, 300);
define_syscall!(PSELECT6, 301);
define_syscall!(PPOLL, 302);
define_syscall!(UNSHARE, 303);
define_syscall!(SPLICE, 304);
define_syscall!(SYNC_FILE_RANGE, 305);
define_syscall!(TEE, 306);
define_syscall!(VMSPLICE, 307);
define_syscall!(MOVE_PAGES, 308);
define_syscall!(SET_ROBUST_LIST, 309);
define_syscall!(GET_ROBUST_LIST, 310);
define_syscall!(KEXEC_LOAD, 311);
define_syscall!(GETCPU, 312);
define_syscall!(EPOLL_PWAIT, 313);
define_syscall!(IOPRIO_SET, 314);
define_syscall!(IOPRIO_GET, 315);
define_syscall!(UTIMENSAT, 316);
define_syscall!(SIGNALFD, 317);
define_syscall!(TIMERFD, 318);
define_syscall!(EVENTFD, 319);
define_syscall!(FALLOCATE, 320);
define_syscall!(TIMERFD_CREATE, 321);
define_syscall!(TIMERFD_GETTIME, 322);
define_syscall!(TIMERFD_SETTIME, 323);
define_syscall!(SIGNALFD4, 324);
define_syscall!(EVENTFD2, 325);
define_syscall!(EPOLL_CREATE1, 326);
define_syscall!(DUP3, 327);
define_syscall!(PIPE2, 328);
define_syscall!(INOTIFY_INIT1, 329);
define_syscall!(PREADV, 330);
define_syscall!(PWRITEV, 331);
define_syscall!(RT_TGSIGQUEUEINFO, 332);
define_syscall!(PERF_EVENT_OPEN, 333);
define_syscall!(ACCEPT4, 334);
define_syscall!(RECVMMSG, 335);
define_syscall!(FANOTIFY_INIT, 336);
define_syscall!(FANOTIFY_MARK, 337);
define_syscall!(PRLIMIT64, 338);
define_syscall!(NAME_TO_HANDLE_AT, 339);
define_syscall!(OPEN_BY_HANDLE_AT, 340);
define_syscall!(CLOCK_ADJTIME, 341);
define_syscall!(SYNCFS, 342);
define_syscall!(SENDMMSG, 343);
define_syscall!(SETNS, 344);
define_syscall!(PROCESS_VM_READV, 345);
define_syscall!(PROCESS_VM_WRITEV, 346);
define_syscall!(KCMP, 347);
define_syscall!(FINIT_MODULE, 348);
define_syscall!(SCHED_SETATTR, 349);
define_syscall!(SCHED_GETATTR, 350);
define_syscall!(RENAMEAT2, 351);
define_syscall!(SECCOMP, 352);
define_syscall!(GETRANDOM, 353);
define_syscall!(MEMFD_CREATE, 354);
define_syscall!(BPF, 355);
define_syscall!(EXECVEAT, 356);
define_syscall!(USERFAULTFD, 357);
define_syscall!(MEMBARRIER, 358);
define_syscall!(MLOCK2, 359);
define_syscall!(COPY_FILE_RANGE, 360);
define_syscall!(PREADV2, 361);
define_syscall!(PWRITEV2, 362);
define_syscall!(PKEY_MPROTECT, 363);
define_syscall!(PKEY_ALLOC, 364);
define_syscall!(PKEY_FREE, 365);
define_syscall!(STATX, 366);
define_syscall!(RSEQ, 367);
define_syscall!(IO_PGETEVENTS, 368);

// non-existent syscalls, will not be called or matched
pub const SYS_NEWFSTATAT: usize = 0;

// custom temporary syscall
pub const SYS_MAP_PCI_DEVICE: usize = 999;
pub const SYS_GET_PADDR: usize = 998;
