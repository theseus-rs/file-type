use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123377812: FileType = FileType {
    file_format: &FileFormat {
        id: 123_377_812,
        source_type: SourceType::Wikidata,
        name: "Lightwright Library file",
        extensions: &["lwb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
