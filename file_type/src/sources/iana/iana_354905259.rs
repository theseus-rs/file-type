use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_354905259: FileType = FileType {
    file_format: &FileFormat {
        id: 354_905_259,
        source_type: SourceType::Iana,
        name: "sbml+xml",
        extensions: &[],
        media_types: &["application/sbml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
