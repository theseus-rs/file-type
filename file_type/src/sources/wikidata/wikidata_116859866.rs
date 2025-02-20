use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116859866: FileType = FileType {
    file_format: &FileFormat {
        id: 116_859_866,
        source_type: SourceType::Wikidata,
        name: "Lesson File",
        extensions: &["lsn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
