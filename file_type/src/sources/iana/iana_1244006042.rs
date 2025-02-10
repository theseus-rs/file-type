use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1244006042: FileType = FileType {
    file_format: &FileFormat {
        id: 1_244_006_042,
        source_type: SourceType::Iana,
        name: "vnd.etsi.pstn+xml",
        extensions: &[],
        media_types: &["application/vnd.etsi.pstn+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
