use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_521782095: FileType = FileType {
    file_format: &FileFormat {
        id: 521_782_095,
        source_type: SourceType::Iana,
        name: "vnd.Mobius.MBK",
        extensions: &[],
        media_types: &["application/vnd.Mobius.MBK"],
        signatures: &[],
        related_formats: &[],
    },
};
