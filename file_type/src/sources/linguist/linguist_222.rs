use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_222: FileFormat = FileFormat {
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
};
