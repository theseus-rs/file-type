use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2339108390: FileType = FileType {
    file_format: &FileFormat {
        id: 2_339_108_390,
        source_type: SourceType::Iana,
        name: "emotionml+xml",
        extensions: &[],
        media_types: &["application/emotionml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
