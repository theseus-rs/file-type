use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66439259: FileType = FileType {
    file_format: &FileFormat {
        id: 66_439_259,
        source_type: SourceType::Wikidata,
        name: "WordPerfect Merge Data file format",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
