use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113626396: FileType = FileType {
    file_format: &FileFormat {
        id: 113_626_396,
        source_type: SourceType::Wikidata,
        name: "ScatterShow file",
        extensions: &["scattershow"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
