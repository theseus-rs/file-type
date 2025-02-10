use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_121093196: FileType = FileType {
    file_format: &FileFormat {
        id: 121_093_196,
        source_type: SourceType::Wikidata,
        name: "Punch! Home Suite PRO file",
        extensions: &["pro"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
