checksums(1) -- Tool for making/verifying checksums of directory trees
======================================================================

## SYNOPSIS

`checksums` [OPTIONS] [DIRECTORY]

## DESCRIPTION

Tool for making/verifying checksums of directory trees.

Use the generated checksums to automatically verify file/directory tree
correctness.

## OPTIONS

  -a --algorithm &lt;<algorithm>&gt;

    Set the hashing algorithm to use.

    Supported algorithms: SHA1, SHA2-256, SHA2-512, SHA3-256, SHA3-512, BLAKE,
                          BLAKE2, CRC8, CRC16, CRC32, CRC64, MD5, MD6-128,
                          MD6-256, MD6-512, XOR8

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

    One thread can hash one file at a time, potentially speeding up hashing up to `jobs` times.

    No/empty value: # of CPU threads. -1: Infinite

  [DIRECTORY]

    Directory to create/verify hash for. Default: current workdir.

## EXAMPLES

  `examples` [`-v`] [`-f` *infile*]

    Verify the current directory tree against the saved hashes.

    `-v` is not necessary as it's the default.

    *infile* defaults to "`DIRECTORY`.hash"

  `examples` `-c` [`-f` *outfile*] [`--force`]

    Create hashes of the current directory tree for later verification.

    *outfile* defaults to "`DIRECTORY`.hash".

    Use `--force` to override *outfile*.

  `examples` [`-d` *depth*] [`-r`] [`OTHER OPTIONS`]

    Recurse *depth* or infinity directories down.

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

## REPORTING BUGS

&lt;<https://github.com/nabijaczleweli/checksums/issues>&gt;

## SEE ALSO

&lt;<https://github.com/nabijaczleweli/checksums>&gt;
