use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3008497538: FileType = FileType {
    file_format: &FileFormat {
        id: 3_008_497_538,
        source_type: SourceType::Iana,
        name: "vnd.ntt-local.sip-ta_tcp_stream",
        extensions: &[],
        media_types: &["application/vnd.ntt-local.sip-ta_tcp_stream"],
        signatures: &[],
        related_formats: &[],
    },
};
