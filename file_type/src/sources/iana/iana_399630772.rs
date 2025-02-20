use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_399630772: FileType = FileType {
    file_format: &FileFormat {
        id: 399_630_772,
        source_type: SourceType::Iana,
        name: "vnd.ezpix-package",
        extensions: &[],
        media_types: &["application/vnd.ezpix-package"],
        signatures: &[],
        related_formats: &[],
    },
};
