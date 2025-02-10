use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117485673: FileType = FileType {
    file_format: &FileFormat {
        id: 117_485_673,
        source_type: SourceType::Wikidata,
        name: "Audacity Project File (Early)",
        extensions: &["aup"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
