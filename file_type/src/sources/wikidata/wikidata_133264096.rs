use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133264096: FileType = FileType {
    file_format: &FileFormat {
        id: 133_264_096,
        source_type: SourceType::Wikidata,
        name: "Plextalk Project File (imdn)",
        extensions: &["imdn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
