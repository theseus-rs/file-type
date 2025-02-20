use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3467211893: FileType = FileType {
    file_format: &FileFormat {
        id: 3_467_211_893,
        source_type: SourceType::Iana,
        name: "application/trust-mark-delegation+jwt",
        extensions: &[],
        media_types: &["application/trust-mark-delegation+jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
