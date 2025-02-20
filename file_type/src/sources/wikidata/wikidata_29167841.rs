use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29167841: FileType = FileType {
    file_format: &FileFormat {
        id: 29_167_841,
        source_type: SourceType::Wikidata,
        name: "Outerra",
        extensions: &["otx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
