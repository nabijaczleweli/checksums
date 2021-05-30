checksums(1) -- Tool for making/verifying checksums of directory trees
======================================================================

## SYNOPSIS

`checksums` [OPTIONS] [DIRECTORY]

## DESCRIPTION

Tool for making/verifying checksums of directory trees.

Use the generated checksums to automatically verify file/directory tree
correctness.

All output is wrapped to 80 columns.

Exit values and possible errors:

    1   - option parsing error
    2   - hash lengths differ between selected and saved
    3   - failed to parse hashes file
    N+3 - N files didn't match

## OPTIONS

  -a --algorithm &lt;<algorithm>&gt;

    Set the hashing algorithm to use, case-insensitive.

    Supported algorithms: SHA1, SHA2-256, SHA2-512, SHA3-256, SHA3-512, BLAKE,
                          BLAKE2B, BLAKE2S, BLAKE3, CRC8, CRC16, CRC32, CRC64,
                          MD5, MD6-128, MD6-256, MD6-512, XOR8

    BLAKE2 is equivalent to BLAKE2B for compatibility.

  -c --create

    Create directory hashes, rather than verifying them.

    Directory hashes are output to the output file, which, if not specified, will
    be "`DIRECTORY`.hash".

    Will fail if the output file already exists and `--force` is not specified.

    Exclusive with `--verify`. Overrides `--verify`.

  -v --verify

    Verify directory hashes. Default.

    Exclusive with `--create`. Overrides `--create`.

  -d --depth &lt;<depth>&gt;

    Set max recursion depth to `depth`. Default: 0.

    Exclusive with `--recursive`. Overrides `--recursive`.

  -r --recursive

    Set max recursion depth to infinity.

    Exclusive with `--depth`. Overrides `--depth`.

  --force

    Override output file in `--create` mode. No meaning in `--verify` mode.

  --follow-symlinks

      Recurse down symlinks. Default.

  --no-follow-symlinks

      Don't recurse down symlinks.

  -i --ignore &lt;<filename[,filename2][,filename3][,filenameN]...>&gt;...

    Add filename(s) to ignored files list. Default: none.

    Ignored files are marked as such.

    Accepted multiple times.

  -j --jobs [jobs]

    Amount of threads used for hashing. Default: # of CPU threads

    One thread can hash one file at a time, potentially speeding up hashing
    up to `jobs` times.

    No/empty value: # of CPU threads. -1: Infinite

  [DIRECTORY]

    Directory to create/verify hash for. Default: current workdir.

## EXAMPLES

  `checksums` [`-v`] [`-f` *infile*]

    Verify the current directory tree against the saved hashes.

    `-v` is not necessary as it's the default.

    *infile* defaults to "`DIRECTORY`.hash"

    Example output:
      File added: "file_that_was_not_here_before"
      File removed: "file_that_was_here_before_but_not_now"
      File ignored: "file_specified_with_ignore_now_or_during_creation"

      File "file_that_did_not_change" matches
      File "changed_file" doesn't match
        Was: 8313958F86F7B15D4775D12886D479C1CFAAA111
        Is : FCFC1548B30B5ACB25A7421D068E12F07DF74DCC

  `checksums` `-c` [`-f` *outfile*] [`--force`]

    Create hashes of the current directory tree for later verification.

    *outfile* defaults to "`DIRECTORY`.hash".

    Use `--force` to override *outfile*.

     Example output:
       FILE 722 / 722 [===============================================] 100.00 %

     *outfile* contents:
       a_file.txt      8313958F86F7B15D4775D12886D479C1CFAAA111
       *outfile*.hash  ----------------------------------------
       different_file  8D742C1F2D39434771039E98AD854C72F91FCCA5

  `checksums` [`-d` *depth*] [`-r`] [`OTHER OPTIONS`]

    Recurse *depth* or infinity directories down.

    Example output for *depth*=2:
      File "dir1/dir2/file" matches
      File "dir1/file" matches
      File "file" matches

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;,
           Zachary Dremann &lt;<dremann@gmail.com>&gt;,
           Chris Moore,
           Daniel Alley &lt;<dalley@redhat.com>&gt;,
       and Paul Bragin &lt;<zeusmods@protonmail.com>&gt;

## REPORTING BUGS

&lt;<https://github.com/nabijaczleweli/checksums/issues>&gt;

## SEE ALSO

&lt;<https://github.com/nabijaczleweli/checksums>&gt;
