use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1081730756: FileType = FileType {
    file_format: &FileFormat {
        id: 1_081_730_756,
        source_type: SourceType::Iana,
        name: "vnd.oma.bcast.sgdd+xml",
        extensions: &[],
        media_types: &["application/vnd.oma.bcast.sgdd+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
