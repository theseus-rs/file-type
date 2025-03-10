use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_17005053: FileType = FileType {
    file_format: &FileFormat {
        id: 17_005_053,
        source_type: SourceType::Wikidata,
        name: "B1",
        extensions: &[],
        media_types: &["application/x-b1"],
        signatures: &[],
        related_formats: &[],
    },
};
