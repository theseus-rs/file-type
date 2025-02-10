use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_612669833: FileType = FileType {
    file_format: &FileFormat {
        id: 612_669_833,
        source_type: SourceType::Linguist,
        name: "Roff Manpage",
        extensions: &[
            "1", "1in", "1m", "1x", "2", "3", "3in", "3m", "3p", "3pm", "3qt", "3x", "4", "5", "6",
            "7", "8", "9", "man", "mdoc",
        ],
        media_types: &["text/troff"],
        signatures: &[],
        related_formats: &[],
    },
};
