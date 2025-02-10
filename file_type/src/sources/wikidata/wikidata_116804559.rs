use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116804559: FileType = FileType {
    file_format: &FileFormat {
        id: 116_804_559,
        source_type: SourceType::Wikidata,
        name: "TimeWiz Project File",
        extensions: &["twz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
