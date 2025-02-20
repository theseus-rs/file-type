use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1934041989: FileType = FileType {
    file_format: &FileFormat {
        id: 1_934_041_989,
        source_type: SourceType::Iana,
        name: "vnd.ms-xpsdocument",
        extensions: &[],
        media_types: &["application/vnd.ms-xpsdocument"],
        signatures: &[],
        related_formats: &[],
    },
};
