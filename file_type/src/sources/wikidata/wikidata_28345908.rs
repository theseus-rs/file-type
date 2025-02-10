use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28345908: FileType = FileType {
    file_format: &FileFormat {
        id: 28_345_908,
        source_type: SourceType::Wikidata,
        name: "Apple Preferred",
        extensions: &["apf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
