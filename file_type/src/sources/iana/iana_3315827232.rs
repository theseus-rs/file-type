use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3315827232: FileType = FileType {
    file_format: &FileFormat {
        id: 3_315_827_232,
        source_type: SourceType::Iana,
        name: "vnd.oasis.opendocument.text-template",
        extensions: &[],
        media_types: &["application/vnd.oasis.opendocument.text-template"],
        signatures: &[],
        related_formats: &[],
    },
};
