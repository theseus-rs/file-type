use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27526471: FileType = FileType {
    file_format: &FileFormat {
        id: 27_526_471,
        source_type: SourceType::Wikidata,
        name: "Broadcast Wave Format, version 1",
        extensions: &["wav"],
        media_types: &["audio/x-wav"],
        signatures: &[],
        related_formats: &[],
    },
};
