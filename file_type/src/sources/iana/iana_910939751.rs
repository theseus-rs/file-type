use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_910939751: FileType = FileType {
    file_format: &FileFormat {
        id: 910_939_751,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.mc-signalling-ear",
        extensions: &[],
        media_types: &["application/vnd.3gpp.mc-signalling-ear"],
        signatures: &[],
        related_formats: &[],
    },
};
