use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
