use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113661747: FileType = FileType {
    file_format: &FileFormat {
        id: 113_661_747,
        source_type: SourceType::Wikidata,
        name: "SciFax file",
        extensions: &["sci"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
