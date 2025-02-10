use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3217232403: FileType = FileType {
    file_format: &FileFormat {
        id: 3_217_232_403,
        source_type: SourceType::Iana,
        name: "vnd.ntt-local.sip-ta_remote",
        extensions: &[],
        media_types: &["application/vnd.ntt-local.sip-ta_remote"],
        signatures: &[],
        related_formats: &[],
    },
};
