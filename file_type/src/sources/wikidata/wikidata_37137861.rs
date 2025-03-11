use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_37137861: FileType = FileType {
    file_format: &FileFormat {
        id: 37_137_861,
        source_type: SourceType::Wikidata,
        name: "Additive Manufacturing File Format",
        extensions: &[],
        media_types: &["application/x-amf"],
        signatures: &[],
        related_formats: &[],
    },
};
