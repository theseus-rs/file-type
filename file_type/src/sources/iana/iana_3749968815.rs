use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3749968815: FileType = FileType {
    file_format: &FileFormat {
        id: 3_749_968_815,
        source_type: SourceType::Iana,
        name: "vnd.foritech.container",
        extensions: &[],
        media_types: &["application/vnd.foritech.container"],
        signatures: &[],
        related_formats: &[],
    },
};
