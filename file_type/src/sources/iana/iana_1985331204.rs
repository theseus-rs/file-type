use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1985331204: FileType = FileType {
    file_format: &FileFormat {
        id: 1_985_331_204,
        source_type: SourceType::Iana,
        name: "vnd.novadigm.EDX",
        extensions: &[],
        media_types: &["application/vnd.novadigm.EDX"],
        signatures: &[],
        related_formats: &[],
    },
};
