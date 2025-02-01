use crate::format::FileFormat;

pub(crate) const LINGUIST_346: FileFormat = FileFormat {
    id: 346,
    puid: "linguist/346",
    name: "Shell",
    extensions: &[
        "bash",
        "bats",
        "cgi",
        "command",
        "fcgi",
        "ksh",
        "sh",
        "sh.in",
        "tmux",
        "tool",
        "trigger",
        "zsh",
        "zsh-theme",
    ],
    media_types: &["text/x-sh"],
    internal_signatures: &[],
    related_formats: &[],
};
