use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59851506: FileType = FileType {
    file_format: &FileFormat {
        id: 59_851_506,
        source_type: SourceType::Wikidata,
        name: "DROID File Collection File Format",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
