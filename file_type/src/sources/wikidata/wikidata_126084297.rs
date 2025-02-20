use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126084297: FileType = FileType {
    file_format: &FileFormat {
        id: 126_084_297,
        source_type: SourceType::Wikidata,
        name: "SPIR-V file",
        extensions: &["spirv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
