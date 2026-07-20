use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138098504: FileType = FileType {
    file_format: &FileFormat {
        id: 138_098_504,
        source_type: SourceType::Wikidata,
        name: "Trim Command file",
        extensions: &["tcf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
