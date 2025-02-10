use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116859922: FileType = FileType {
    file_format: &FileFormat {
        id: 116_859_922,
        source_type: SourceType::Wikidata,
        name: "Test File",
        extensions: &["tst"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
