use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1730947935: FileType = FileType {
    file_format: &FileFormat {
        id: 1_730_947_935,
        source_type: SourceType::Iana,
        name: "vnd.ntt-local.content-share",
        extensions: &[],
        media_types: &["application/vnd.ntt-local.content-share"],
        signatures: &[],
        related_formats: &[],
    },
};
