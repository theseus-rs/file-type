use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_4545331: FileType = FileType {
    file_format: &FileFormat {
        id: 4_545_331,
        source_type: SourceType::Wikidata,
        name: ".3ds",
        extensions: &["3ds"],
        media_types: &["application/x-3ds", "image/x-3ds"],
        signatures: &[],
        related_formats: &[],
    },
};
