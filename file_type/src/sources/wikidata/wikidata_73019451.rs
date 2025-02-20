use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_73019451: FileType = FileType {
    file_format: &FileFormat {
        id: 73_019_451,
        source_type: SourceType::Wikidata,
        name: "Picture Publisher Bitmap, version 5.0",
        extensions: &["pp5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
