use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3248030970: FileType = FileType {
    file_format: &FileFormat {
        id: 3_248_030_970,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.ngap",
        extensions: &[],
        media_types: &["application/vnd.3gpp.ngap"],
        signatures: &[],
        related_formats: &[],
    },
};
