use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111182231: FileType = FileType {
    file_format: &FileFormat {
        id: 111_182_231,
        source_type: SourceType::Wikidata,
        name: "ActionScript Communication File",
        extensions: &["asc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
