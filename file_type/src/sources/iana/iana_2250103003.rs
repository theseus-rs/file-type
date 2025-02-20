use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2250103003: FileType = FileType {
    file_format: &FileFormat {
        id: 2_250_103_003,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.s1ap",
        extensions: &[],
        media_types: &["application/vnd.3gpp.s1ap"],
        signatures: &[],
        related_formats: &[],
    },
};
