use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_126485053: FileType = FileType {
    file_format: &FileFormat {
        id: 126_485_053,
        source_type: SourceType::Wikidata,
        name: "Omnis Sudio Project Library file",
        extensions: &["lbs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
