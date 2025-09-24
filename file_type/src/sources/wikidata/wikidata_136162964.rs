use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136162964: FileType = FileType {
    file_format: &FileFormat {
        id: 136_162_964,
        source_type: SourceType::Wikidata,
        name: "Moose file",
        extensions: &["mse"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
