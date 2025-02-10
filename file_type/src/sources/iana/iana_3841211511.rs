use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3841211511: FileType = FileType {
    file_format: &FileFormat {
        id: 3_841_211_511,
        source_type: SourceType::Iana,
        name: "dvcs",
        extensions: &[],
        media_types: &["application/dvcs"],
        signatures: &[],
        related_formats: &[],
    },
};
