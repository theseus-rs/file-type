use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133804303: FileType = FileType {
    file_format: &FileFormat {
        id: 133_804_303,
        source_type: SourceType::Wikidata,
        name: "Kyss Graphics file",
        extensions: &["kyg"],
        media_types: &["image/x-kyss-graphics"],
        signatures: &[],
        related_formats: &[],
    },
};
