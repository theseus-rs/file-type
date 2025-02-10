use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61963212: FileType = FileType {
    file_format: &FileFormat {
        id: 61_963_212,
        source_type: SourceType::Wikidata,
        name: "Lotus WordPro Document",
        extensions: &["lwp"],
        media_types: &["application/lwp", "application/vnd.lotus-wordpro"],
        signatures: &[],
        related_formats: &[],
    },
};
