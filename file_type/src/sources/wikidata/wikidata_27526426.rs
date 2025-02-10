use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27526426: FileType = FileType {
    file_format: &FileFormat {
        id: 27_526_426,
        source_type: SourceType::Wikidata,
        name: "Broadcast Wave Format, version 0",
        extensions: &["wav"],
        media_types: &["audio/x-wav"],
        signatures: &[],
        related_formats: &[],
    },
};
