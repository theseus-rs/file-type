use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2210814693: FileType = FileType {
    file_format: &FileFormat {
        id: 2_210_814_693,
        source_type: SourceType::Iana,
        name: "vnd.aplextor.warrp+json",
        extensions: &[],
        media_types: &["application/vnd.aplextor.warrp+json"],
        signatures: &[],
        related_formats: &[],
    },
};
