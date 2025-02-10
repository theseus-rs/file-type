use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_85513340: FileType = FileType {
    file_format: &FileFormat {
        id: 85_513_340,
        source_type: SourceType::Wikidata,
        name: "Cindex Document, version 3",
        extensions: &["ucdx", "utpl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
