use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125809623: FileType = FileType {
    file_format: &FileFormat {
        id: 125_809_623,
        source_type: SourceType::Wikidata,
        name: "XZ Compressed Tar Archive",
        extensions: &["txz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
