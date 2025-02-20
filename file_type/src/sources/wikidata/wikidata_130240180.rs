use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130240180: FileType = FileType {
    file_format: &FileFormat {
        id: 130_240_180,
        source_type: SourceType::Wikidata,
        name: "Liquid template file",
        extensions: &["liquid"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
