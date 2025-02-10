use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967082: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_082,
        source_type: SourceType::Wikidata,
        name: "David Whittaker",
        extensions: &["dw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
