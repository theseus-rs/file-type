use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967191: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_191,
        source_type: SourceType::Wikidata,
        name: "Grave Composer module",
        extensions: &["wow"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
