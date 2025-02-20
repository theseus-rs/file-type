use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117104232: FileType = FileType {
    file_format: &FileFormat {
        id: 117_104_232,
        source_type: SourceType::Wikidata,
        name: "Picture it! Publishing File",
        extensions: &["php"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
