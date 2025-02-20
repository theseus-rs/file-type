use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
