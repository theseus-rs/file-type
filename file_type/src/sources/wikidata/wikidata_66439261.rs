use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66439261: FileType = FileType {
    file_format: &FileFormat {
        id: 66_439_261,
        source_type: SourceType::Wikidata,
        name: "WordPerfect Merge Forms file format",
        extensions: &["frm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
