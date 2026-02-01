export def search [
    pattern: string
    --file (-f): path
    --list-files (-l)
    -A: int
    -C: int
] {
    mut flags = [
        "--ignore-case"
        "--no-ignore"
    ]

    if ($A | is-not-empty) {
        $flags ++= ["-A" $A]
    }

    if ($C | is-not-empty) {
        $flags ++= ["-C" $C]
    }

    if $list_files {
        return (rg ...$flags -l $pattern)
    }

    if ($file | is-not-empty) {
        open $file | to json --indent 4 | rg ...$flags $pattern
    } else {
        rg ...$flags $pattern
    }
}