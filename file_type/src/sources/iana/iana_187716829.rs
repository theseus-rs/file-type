use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_187716829: FileType = FileType {
    file_format: &FileFormat {
        id: 187_716_829,
        source_type: SourceType::Iana,
        name: "vnd.cluetrust.cartomobile-config",
        extensions: &[],
        media_types: &["application/vnd.cluetrust.cartomobile-config"],
        signatures: &[],
        related_formats: &[],
    },
};
