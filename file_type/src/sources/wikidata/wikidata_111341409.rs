use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111341409: FileType = FileType {
    file_format: &FileFormat {
        id: 111_341_409,
        source_type: SourceType::Wikidata,
        name: "Yamaha EX5 'all' format",
        extensions: &["s1a"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
