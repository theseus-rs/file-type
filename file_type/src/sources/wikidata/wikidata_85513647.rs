use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_85513647: FileType = FileType {
    file_format: &FileFormat {
        id: 85_513_647,
        source_type: SourceType::Wikidata,
        name: "Cindex Document, version 4",
        extensions: &["ucdx", "utpl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
