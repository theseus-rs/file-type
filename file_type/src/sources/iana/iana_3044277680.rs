use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3044277680: FileType = FileType {
    file_format: &FileFormat {
        id: 3_044_277_680,
        source_type: SourceType::Iana,
        name: "gnap-binding-jws",
        extensions: &[],
        media_types: &["application/gnap-binding-jws"],
        signatures: &[],
        related_formats: &[],
    },
};
