use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_222: FileType = FileType {
    file_format: &FileFormat {
        id: 222,
        source_type: SourceType::Linguist,
        name: "Markdown",
        extensions: &[
            "livemd", "markdown", "md", "mdown", "mdwn", "mkd", "mkdn", "mkdown", "ronn", "scd",
            "workbook",
        ],
        media_types: &["text/x-gfm"],
        signatures: &[],
        related_formats: &[],
    },
};
