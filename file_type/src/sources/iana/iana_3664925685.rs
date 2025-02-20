use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3664925685: FileType = FileType {
    file_format: &FileFormat {
        id: 3_664_925_685,
        source_type: SourceType::Iana,
        name: "vnd.espass-espass+zip",
        extensions: &[],
        media_types: &["application/vnd.espass-espass+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
