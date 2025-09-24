use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136088459: FileType = FileType {
    file_format: &FileFormat {
        id: 136_088_459,
        source_type: SourceType::Wikidata,
        name: "Celtx Script Document file",
        extensions: &["cxscript"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
