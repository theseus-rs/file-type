use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2333343507: FileType = FileType {
    file_format: &FileFormat {
        id: 2_333_343_507,
        source_type: SourceType::Iana,
        name: "samlassertion+xml",
        extensions: &[],
        media_types: &["application/samlassertion+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
