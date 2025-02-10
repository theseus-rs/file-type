use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4102381814: FileType = FileType {
    file_format: &FileFormat {
        id: 4_102_381_814,
        source_type: SourceType::Iana,
        name: "vnd.etsi.iptvprofile+xml",
        extensions: &[],
        media_types: &["application/vnd.etsi.iptvprofile+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
