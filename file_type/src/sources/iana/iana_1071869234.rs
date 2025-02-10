use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1071869234: FileType = FileType {
    file_format: &FileFormat {
        id: 1_071_869_234,
        source_type: SourceType::Iana,
        name: "relax-ng-compact-syntax",
        extensions: &[],
        media_types: &["application/relax-ng-compact-syntax"],
        signatures: &[],
        related_formats: &[],
    },
};
