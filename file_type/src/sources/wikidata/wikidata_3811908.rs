use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_3811908: FileType = FileType {
    file_format: &FileFormat {
        id: 3_811_908,
        source_type: SourceType::Wikidata,
        name: "karaoke file",
        extensions: &["kar"],
        media_types: &["audio/midi"],
        signatures: &[],
        related_formats: &[],
    },
};
