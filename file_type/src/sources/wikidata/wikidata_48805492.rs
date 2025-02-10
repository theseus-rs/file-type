use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48805492: FileType = FileType {
    file_format: &FileFormat {
        id: 48_805_492,
        source_type: SourceType::Wikidata,
        name: "ChiWriter Document",
        extensions: &["chi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
