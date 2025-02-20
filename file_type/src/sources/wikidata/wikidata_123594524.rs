use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123594524: FileType = FileType {
    file_format: &FileFormat {
        id: 123_594_524,
        source_type: SourceType::Wikidata,
        name: "TibetDoc Word Document",
        extensions: &["dct"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
