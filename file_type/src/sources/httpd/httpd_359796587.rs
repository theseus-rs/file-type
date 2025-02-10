use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_359796587: FileType = FileType {
    file_format: &FileFormat {
        id: 359_796_587,
        source_type: SourceType::Httpd,
        name: "noblenet sealer",
        extensions: &["nns"],
        media_types: &["application/vnd.noblenet-sealer"],
        signatures: &[],
        related_formats: &[],
    },
};
