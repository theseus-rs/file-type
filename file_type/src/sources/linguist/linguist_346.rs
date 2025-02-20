use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_346: FileType = FileType {
    file_format: &FileFormat {
        id: 346,
        source_type: SourceType::Linguist,
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
        signatures: &[],
        related_formats: &[],
    },
};
