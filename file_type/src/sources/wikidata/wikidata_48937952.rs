use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48937952: FileType = FileType {
    file_format: &FileFormat {
        id: 48_937_952,
        source_type: SourceType::Wikidata,
        name: "descript.ion",
        extensions: &["ion"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
