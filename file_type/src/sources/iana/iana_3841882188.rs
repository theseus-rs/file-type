use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3841882188: FileType = FileType {
    file_format: &FileFormat {
        id: 3_841_882_188,
        source_type: SourceType::Iana,
        name: "vnd.wv.csp+wbxml",
        extensions: &[],
        media_types: &["application/vnd.wv.csp+wbxml"],
        signatures: &[],
        related_formats: &[],
    },
};
