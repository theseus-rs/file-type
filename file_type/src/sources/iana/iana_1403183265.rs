use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1403183265: FileType = FileType {
    file_format: &FileFormat {
        id: 1_403_183_265,
        source_type: SourceType::Iana,
        name: "vnd.americandynamics.acc",
        extensions: &[],
        media_types: &["application/vnd.americandynamics.acc"],
        signatures: &[],
        related_formats: &[],
    },
};
