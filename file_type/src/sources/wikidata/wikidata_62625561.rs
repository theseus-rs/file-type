use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_62625561: FileType = FileType {
    file_format: &FileFormat {
        id: 62_625_561,
        source_type: SourceType::Wikidata,
        name: "Bash script",
        extensions: &["bash", "bsh", "csh", "sh"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
