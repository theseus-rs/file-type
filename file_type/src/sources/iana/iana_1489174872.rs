use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1489174872: FileType = FileType {
    file_format: &FileFormat {
        id: 1_489_174_872,
        source_type: SourceType::Iana,
        name: "vnd.cryptomator.vault",
        extensions: &[],
        media_types: &["application/vnd.cryptomator.vault"],
        signatures: &[],
        related_formats: &[],
    },
};
