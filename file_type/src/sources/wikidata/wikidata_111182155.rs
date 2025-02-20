use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111182155: FileType = FileType {
    file_format: &FileFormat {
        id: 111_182_155,
        source_type: SourceType::Wikidata,
        name: "Dreamweaver Library Item",
        extensions: &["lbi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
