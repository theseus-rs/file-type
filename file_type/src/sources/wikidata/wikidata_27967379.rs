use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967379: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_379,
        source_type: SourceType::Wikidata,
        name: "B00",
        extensions: &["b00"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
