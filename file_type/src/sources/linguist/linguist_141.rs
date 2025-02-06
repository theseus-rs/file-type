use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_141: FileFormat = FileFormat {
    id: 141,
    source_type: SourceType::Linguist,
    name: "Roff",
    extensions: &[
        "1", "1in", "1m", "1x", "2", "3", "3in", "3m", "3p", "3pm", "3qt", "3x", "4", "5", "6",
        "7", "8", "9", "l", "man", "mdoc", "me", "ms", "n", "nr", "rno", "roff", "tmac",
    ],
    media_types: &["text/troff"],
    signatures: &[],
    related_formats: &[],
};
