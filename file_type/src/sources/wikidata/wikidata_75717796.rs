use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_75717796: FileType = FileType {
    file_format: &FileFormat {
        id: 75_717_796,
        source_type: SourceType::Wikidata,
        name: "USRobotics firmware",
        extensions: &["usr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
