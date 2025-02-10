use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1789479840: FileType = FileType {
    file_format: &FileFormat {
        id: 1_789_479_840,
        source_type: SourceType::Iana,
        name: "vnd.vividence.scriptfile",
        extensions: &[],
        media_types: &["application/vnd.vividence.scriptfile"],
        signatures: &[],
        related_formats: &[],
    },
};
