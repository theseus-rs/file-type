use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1684578751: FileType = FileType {
    file_format: &FileFormat {
        id: 1_684_578_751,
        source_type: SourceType::Iana,
        name: "fwdred",
        extensions: &[],
        media_types: &["audio/fwdred"],
        signatures: &[],
        related_formats: &[],
    },
};
