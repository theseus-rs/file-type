use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_118583163: FileType = FileType {
    file_format: &FileFormat {
        id: 118_583_163,
        source_type: SourceType::Wikidata,
        name: "Kinetic Project",
        extensions: &["kin"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
