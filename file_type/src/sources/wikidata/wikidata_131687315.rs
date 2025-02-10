use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131687315: FileType = FileType {
    file_format: &FileFormat {
        id: 131_687_315,
        source_type: SourceType::Wikidata,
        name: "HTML Email Markup Language format",
        extensions: &["heml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
