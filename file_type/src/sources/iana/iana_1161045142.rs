use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1161045142: FileType = FileType {
    file_format: &FileFormat {
        id: 1_161_045_142,
        source_type: SourceType::Iana,
        name: "yang",
        extensions: &[],
        media_types: &["application/yang"],
        signatures: &[],
        related_formats: &[],
    },
};
