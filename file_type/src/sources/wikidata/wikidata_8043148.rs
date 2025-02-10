use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_8043148: FileType = FileType {
    file_format: &FileFormat {
        id: 8_043_148,
        source_type: SourceType::Wikidata,
        name: "Xara Flare",
        extensions: &["xar"],
        media_types: &["application/vnd.xara"],
        signatures: &[],
        related_formats: &[],
    },
};
