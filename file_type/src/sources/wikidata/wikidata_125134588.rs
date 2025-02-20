use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125134588: FileType = FileType {
    file_format: &FileFormat {
        id: 125_134_588,
        source_type: SourceType::Wikidata,
        name: "YAM Unique ID Listing",
        extensions: &["uidl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
