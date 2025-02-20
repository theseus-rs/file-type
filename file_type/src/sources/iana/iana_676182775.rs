use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_676182775: FileType = FileType {
    file_format: &FileFormat {
        id: 676_182_775,
        source_type: SourceType::Iana,
        name: "vnd.quobject-quoxdocument",
        extensions: &[],
        media_types: &["application/vnd.quobject-quoxdocument"],
        signatures: &[],
        related_formats: &[],
    },
};
