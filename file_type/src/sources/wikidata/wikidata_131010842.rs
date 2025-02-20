use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131010842: FileType = FileType {
    file_format: &FileFormat {
        id: 131_010_842,
        source_type: SourceType::Wikidata,
        name: "Smithy file format",
        extensions: &["smithy"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
