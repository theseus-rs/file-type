use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1211261047: FileType = FileType {
    file_format: &FileFormat {
        id: 1_211_261_047,
        source_type: SourceType::Iana,
        name: "cid",
        extensions: &[],
        media_types: &["application/cid"],
        signatures: &[],
        related_formats: &[],
    },
};
