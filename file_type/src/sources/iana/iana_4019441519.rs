use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4019441519: FileType = FileType {
    file_format: &FileFormat {
        id: 4_019_441_519,
        source_type: SourceType::Iana,
        name: "vnd.airzip.filesecure.azs",
        extensions: &[],
        media_types: &["application/vnd.airzip.filesecure.azs"],
        signatures: &[],
        related_formats: &[],
    },
};
