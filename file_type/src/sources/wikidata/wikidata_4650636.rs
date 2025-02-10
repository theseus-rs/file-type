use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_4650636: FileType = FileType {
    file_format: &FileFormat {
        id: 4_650_636,
        source_type: SourceType::Wikidata,
        name: "ACE file format",
        extensions: &["ace"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
