use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3880584458: FileType = FileType {
    file_format: &FileFormat {
        id: 3_880_584_458,
        source_type: SourceType::Iana,
        name: "vnd.wv.csp+xml",
        extensions: &[],
        media_types: &["application/vnd.wv.csp+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
