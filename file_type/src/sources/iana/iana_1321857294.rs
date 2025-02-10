use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1321857294: FileType = FileType {
    file_format: &FileFormat {
        id: 1_321_857_294,
        source_type: SourceType::Iana,
        name: "vnd.iptc.g2.catalogitem+xml",
        extensions: &[],
        media_types: &["application/vnd.iptc.g2.catalogitem+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
