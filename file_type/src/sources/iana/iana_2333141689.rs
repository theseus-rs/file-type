use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2333141689: FileType = FileType {
    file_format: &FileFormat {
        id: 2_333_141_689,
        source_type: SourceType::Iana,
        name: "vnd.gnu.taler.merchant+json",
        extensions: &[],
        media_types: &["application/vnd.gnu.taler.merchant+json"],
        signatures: &[],
        related_formats: &[],
    },
};
