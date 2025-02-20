use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_891056696: FileType = FileType {
    file_format: &FileFormat {
        id: 891_056_696,
        source_type: SourceType::Iana,
        name: "vnd.MFER",
        extensions: &[],
        media_types: &["application/vnd.MFER"],
        signatures: &[],
        related_formats: &[],
    },
};
