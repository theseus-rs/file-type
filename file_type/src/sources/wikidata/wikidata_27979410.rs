use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27979410: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_410,
        source_type: SourceType::Wikidata,
        name: "Binary Text",
        extensions: &["bin"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
