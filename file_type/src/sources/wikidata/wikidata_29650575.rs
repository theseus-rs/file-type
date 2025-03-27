use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29650575: FileType = FileType {
    file_format: &FileFormat {
        id: 29_650_575,
        source_type: SourceType::Wikidata,
        name: "Pages",
        extensions: &["pages", "pages.zip"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
