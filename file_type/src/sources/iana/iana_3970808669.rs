use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3970808669: FileType = FileType {
    file_format: &FileFormat {
        id: 3_970_808_669,
        source_type: SourceType::Iana,
        name: "pkix-crl",
        extensions: &[],
        media_types: &["application/pkix-crl"],
        signatures: &[],
        related_formats: &[],
    },
};
