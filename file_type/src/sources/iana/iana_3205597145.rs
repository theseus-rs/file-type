use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3205597145: FileType = FileType {
    file_format: &FileFormat {
        id: 3_205_597_145,
        source_type: SourceType::Iana,
        name: "vnd.sosi",
        extensions: &[],
        media_types: &["text/vnd.sosi"],
        signatures: &[],
        related_formats: &[],
    },
};
