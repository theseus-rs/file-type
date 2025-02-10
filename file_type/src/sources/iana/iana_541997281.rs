use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_541997281: FileType = FileType {
    file_format: &FileFormat {
        id: 541_997_281,
        source_type: SourceType::Iana,
        name: "vnd.opentimestamps.ots",
        extensions: &[],
        media_types: &["application/vnd.opentimestamps.ots"],
        signatures: &[],
        related_formats: &[],
    },
};
