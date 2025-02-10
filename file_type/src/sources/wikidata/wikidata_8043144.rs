use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_8043144: FileType = FileType {
    file_format: &FileFormat {
        id: 8_043_144,
        source_type: SourceType::Wikidata,
        name: "Xar",
        extensions: &["xar"],
        media_types: &["application/vnd.xara"],
        signatures: &[],
        related_formats: &[],
    },
};
