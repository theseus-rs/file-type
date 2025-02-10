use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_340992391: FileType = FileType {
    file_format: &FileFormat {
        id: 340_992_391,
        source_type: SourceType::Iana,
        name: "vnd.xecrets-encrypted",
        extensions: &[],
        media_types: &["application/vnd.xecrets-encrypted"],
        signatures: &[],
        related_formats: &[],
    },
};
