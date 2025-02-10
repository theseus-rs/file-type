use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2019995601: FileType = FileType {
    file_format: &FileFormat {
        id: 2_019_995_601,
        source_type: SourceType::Iana,
        name: "vnd.Mobius.MQY",
        extensions: &[],
        media_types: &["application/vnd.Mobius.MQY"],
        signatures: &[],
        related_formats: &[],
    },
};
