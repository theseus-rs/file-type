use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
