use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_535414097: FileType = FileType {
    file_format: &FileFormat {
        id: 535_414_097,
        source_type: SourceType::Iana,
        name: "statuslist+jwt",
        extensions: &[],
        media_types: &["application/statuslist+jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
