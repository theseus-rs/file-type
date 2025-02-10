use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_129832569: FileType = FileType {
    file_format: &FileFormat {
        id: 129_832_569,
        source_type: SourceType::Wikidata,
        name: "Isabelle file format",
        extensions: &["thy"],
        media_types: &["text/x-isabelle"],
        signatures: &[],
        related_formats: &[],
    },
};
