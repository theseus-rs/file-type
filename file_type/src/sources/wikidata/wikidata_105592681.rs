use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105592681: FileType = FileType {
    file_format: &FileFormat {
        id: 105_592_681,
        source_type: SourceType::Wikidata,
        name: "Guitar Pro 7 tablature",
        extensions: &["gp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
