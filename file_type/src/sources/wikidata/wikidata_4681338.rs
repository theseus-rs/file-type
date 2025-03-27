use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_4681338: FileType = FileType {
    file_format: &FileFormat {
        id: 4_681_338,
        source_type: SourceType::Wikidata,
        name: "Additive Manufacturing File Format standard",
        extensions: &["amf"],
        media_types: &["application/x-amf"],
        signatures: &[],
        related_formats: &[],
    },
};
