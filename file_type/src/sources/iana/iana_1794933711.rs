use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1794933711: FileType = FileType {
    file_format: &FileFormat {
        id: 1_794_933_711,
        source_type: SourceType::Iana,
        name: "jcr-cnd",
        extensions: &[],
        media_types: &["text/jcr-cnd"],
        signatures: &[],
        related_formats: &[],
    },
};
