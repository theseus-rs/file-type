use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131144603: FileType = FileType {
    file_format: &FileFormat {
        id: 131_144_603,
        source_type: SourceType::Wikidata,
        name: "Sophia file format",
        extensions: &["aes"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
