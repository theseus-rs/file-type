use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29000658: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_658,
        source_type: SourceType::Wikidata,
        name: "PTX",
        extensions: &["ptx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
