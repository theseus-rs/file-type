use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_129180035: FileType = FileType {
    file_format: &FileFormat {
        id: 129_180_035,
        source_type: SourceType::Wikidata,
        name: "Fish shell script",
        extensions: &["fish"],
        media_types: &["application/x-fish"],
        signatures: &[],
        related_formats: &[],
    },
};
